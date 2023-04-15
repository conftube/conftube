use crate::AppContext;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder};
use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata};
use openidconnect::reqwest::async_http_client;
use openidconnect::{
    AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse,
};
use serde::Deserialize;
use std::error::Error;

pub struct OpenIDConnectConfig {
    pub(crate) issuer_url: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) redirect_url: String,
}

pub async fn create_client(config: OpenIDConnectConfig) -> Result<CoreClient, Box<dyn Error>> {
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(config.issuer_url).unwrap(),
        async_http_client,
    )
    .await?;

    Ok(CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(config.client_id),
        Some(ClientSecret::new(config.client_secret)),
    )
    .set_redirect_uri(RedirectUrl::new(config.redirect_url)?))
}

pub async fn login(context: Data<AppContext>, session: Session) -> impl Responder {
    let client = context.oidc_client.clone();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge.clone())
        .url();

    session
        .insert("nonce", nonce)
        .expect("Error storing in session");

    session
        .insert("csrf_token", csrf_token)
        .expect("Error storing in session");

    session
        .insert("pkce", pkce_verifier)
        .expect("Error storing in session");

    HttpResponse::TemporaryRedirect()
        .insert_header(("Location", auth_url.to_string()))
        .body("Redirecting ...")
}

#[derive(Debug, Deserialize)]
pub struct AuthCallback {
    code: String,
    state: String,
}

pub async fn auth_callback(
    context: Data<AppContext>,
    session: Session,
    info: web::Query<AuthCallback>,
) -> HttpResponse {
    let stored_nonce = session.get::<Nonce>("nonce").expect("Error fetching nonce");

    let stored_csrf_token = session
        .get::<CsrfToken>("csrf_token")
        .expect("Error fetching nonce");

    let pkce_verifier = session
        .get::<PkceCodeVerifier>("pkce")
        .expect("Error fetching pkce")
        .expect("Error unwrapping pkce");

    let nonce = match stored_nonce {
        None => return HttpResponse::NotFound().body(""),
        Some(nonce) => nonce,
    };

    let stored_csrf_token = match stored_csrf_token {
        None => return HttpResponse::NotFound().body(""),
        Some(csrf_token) => csrf_token,
    };

    // TODO: compare state + csrf_token

    if info.state != *stored_csrf_token.secret() {
        return HttpResponse::NotFound().finish();
    }

    let client = context.oidc_client.clone();
    let token_response = client
        .exchange_code(AuthorizationCode::new(info.code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .expect("Error verifying token");

    let id_token = token_response.id_token().expect("Error getting ID token");

    let claims = id_token
        .claims(&client.id_token_verifier(), &nonce)
        .expect("Error getting claims");

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token_response.access_token(),
            &id_token.signing_alg().unwrap(),
        )
        .expect("Error getting token hash");

        if actual_access_token_hash != *expected_access_token_hash {
            return HttpResponse::Forbidden().finish();
        }
    }

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

use crate::schemas::project_schemas::{NewUser, User};
use crate::AppContext;
use actix_session::{Session, SessionInsertError};
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, ResponseError};
use openidconnect::core::{
    CoreAuthenticationFlow, CoreClient, CoreGenderClaim, CoreProviderMetadata,
    CoreRequestTokenError, CoreTokenResponse,
};
use openidconnect::reqwest::{async_http_client, HttpClientError};
use openidconnect::{
    AccessTokenHash, AuthorizationCode, ClaimsVerificationError, ClientId, ClientSecret, CsrfToken,
    EmptyAdditionalClaims, IdTokenClaims, IssuerUrl, Nonce, OAuth2TokenResponse, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, SigningError, TokenResponse,
};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthState {
    csrf_token: CsrfToken,
    pkce_verifier: PkceCodeVerifier,
    nonce: Nonce,
}

pub async fn login(
    context: Data<AppContext>,
    session: Session,
) -> Result<HttpResponse, SessionInsertError> {
    let client = context.oidc_client.clone();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let mut request = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .set_pkce_challenge(pkce_challenge);

    request = request.add_scope(Scope::new("read".into()));
    request = request.add_scope(Scope::new("write".into()));
    request = request.add_scope(Scope::new("openid".into()));
    request = request.add_scope(Scope::new("profile".into()));
    request = request.add_scope(Scope::new("email".into()));

    let (auth_url, csrf_token, nonce) = request.url();

    session.insert(
        "oidc-request",
        AuthState {
            csrf_token,
            pkce_verifier,
            nonce,
        },
    )?;

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCallback {
    code: AuthorizationCode,
    state: CsrfToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    claims: IdTokenClaims<EmptyAdditionalClaims, CoreGenderClaim>,
    token: CoreTokenResponse,
}

pub async fn auth_callback(
    context: Data<AppContext>,
    session: Session,
    params: web::Query<AuthCallback>,
) -> Result<HttpResponse, AuthCallbackError> {
    let client = context.oidc_client.clone();
    let AuthCallback { code, state } = params.into_inner();

    let AuthState {
        csrf_token,
        pkce_verifier,
        nonce,
    } = session
        .remove_as("oidc-request")
        .ok_or(AuthCallbackError::MissingState)?
        .map_err(|_| AuthCallbackError::MissingState)?;

    if state.secret() != csrf_token.secret() {
        return Err(AuthCallbackError::InvalidState);
    }

    let token = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .map_err(AuthCallbackError::FailedRequestToken)?;

    let id_token = token.id_token().ok_or(AuthCallbackError::MissingIdToken)?;
    let claims = id_token
        .claims(&client.id_token_verifier(), &nonce)
        .map_err(AuthCallbackError::InvalidIdToken)?;

    if let Some(expected_access_token_hash) = claims.access_token_hash() {
        let actual_access_token_hash = AccessTokenHash::from_token(
            token.access_token(),
            &id_token
                .signing_alg()
                .map_err(AuthCallbackError::CreateAccessTokenHash)?,
        )
        .map_err(AuthCallbackError::CreateAccessTokenHash)?;

        if actual_access_token_hash != *expected_access_token_hash {
            return Err(AuthCallbackError::InvalidAccessTokenHash);
        }
    }

    session
        .insert(
            "user_info",
            UserInfo {
                claims: claims.clone(),
                token: token.clone(),
            },
        )
        .map_err(AuthCallbackError::SessionInsert)?;

    let user = NewUser {
        id: None,
        email: claims.email().unwrap().to_string(),
        given_name: claims.given_name().unwrap().get(None).unwrap().to_string(),
        family_name: claims.family_name().unwrap().get(None).unwrap().to_string(),
        picture: claims.picture().unwrap().get(None).unwrap().to_string(),
    };

    let mut conn = context.db_pool.get().unwrap();

    user.register(&mut conn).expect("Error registering user");

    // TODO: implement middleware to check if the session is still valid!
    // TODO: validate token
    // TODO: add auth to all routes except login

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish())
}

#[derive(Debug)]
pub enum AuthCallbackError {
    /// There is no `state` in the user's session
    /// Maybe he hasn't visited [`login`] yet?
    MissingState,

    /// The `state` in the user's session doesn't match the `state` the oidc provider responded with.
    InvalidState,

    /// Failed to request the actual token from the oidc provider
    FailedRequestToken(CoreRequestTokenError<HttpClientError>),

    /// The provider didn't send a id token
    MissingIdToken,

    /// Failed to verify the id token while reading claims
    InvalidIdToken(ClaimsVerificationError),

    /// Error occurring while generating an [`AccessTokenHash`]
    CreateAccessTokenHash(SigningError),

    /// The claims' access token doesn't match the oidc's
    InvalidAccessTokenHash,

    /// Error from [`Session::insert`]
    SessionInsert(SessionInsertError),
}
impl std::fmt::Display for AuthCallbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthCallbackError::MissingState => write!(f, "State is missing from user session"),
            AuthCallbackError::InvalidState => write!(f, "State in user session is invalid"),
            AuthCallbackError::FailedRequestToken(err) => {
                write!(f, "Failed to request token: {err}")
            }
            AuthCallbackError::MissingIdToken => {
                write!(f, "Provider didn't respond with an ID token")
            }
            AuthCallbackError::InvalidIdToken(err) => {
                write!(f, "The ID token didn't pass the verification: {err}")
            }
            AuthCallbackError::CreateAccessTokenHash(err) => {
                write!(f, "Couldn't generate the access token's hash: {err}")
            }
            AuthCallbackError::InvalidAccessTokenHash => {
                write!(f, "The access token's hash doesn't match")
            }
            AuthCallbackError::SessionInsert(err) => {
                write!(f, "Failed to set token in user session: {err}")
            }
        }
    }
}
impl Error for AuthCallbackError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AuthCallbackError::MissingState => None,
            AuthCallbackError::InvalidState => None,
            AuthCallbackError::FailedRequestToken(err) => Some(&*err),
            AuthCallbackError::SessionInsert(err) => Some(err),
            AuthCallbackError::MissingIdToken => None,
            AuthCallbackError::CreateAccessTokenHash(err) => Some(err),
            AuthCallbackError::InvalidAccessTokenHash => None,
            AuthCallbackError::InvalidIdToken(err) => Some(err),
        }
    }
}
impl ResponseError for AuthCallbackError {}

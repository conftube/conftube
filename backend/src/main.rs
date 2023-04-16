use crate::auth::{create_client, OpenIDConnectConfig};
use crate::handler::graphql_handler::{Mutation, ProjectSchema, Query};
use crate::youtube::{Youtube, YoutubeClient};
use actix_files::Files;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::prelude::*;
use diesel::r2d2;
use dotenvy::dotenv;
use openidconnect::core::CoreClient;
use std::sync::Arc;

mod auth;
mod db_schema;
mod handler;
mod schemas;
mod youtube;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
type OidcClient = Arc<CoreClient>;

pub struct AppContext {
    schema: ProjectSchema,
    oidc_client: OidcClient,
    youtube_client: Youtube,
}

impl Clone for AppContext {
    fn clone(&self) -> Self {
        Self {
            youtube_client: self.youtube_client.clone(),
            oidc_client: self.oidc_client.clone(),
            schema: self.schema.clone(),
        }
    }
}

async fn graphql(context: Data<AppContext>, req: GraphQLRequest) -> GraphQLResponse {
    context.schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish())
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
fn initialize_db_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Error building r2d2 pool")
}

async fn initialize_oidc_client() -> OidcClient {
    Arc::new(
        create_client(OpenIDConnectConfig {
            issuer_url: std::env::var("OIDC_ISSUER_URL").expect("OIDC_ISSUER_URL needs to be set"),
            client_id: std::env::var("OIDC_CLIENT_ID").expect("OIDC_CLIENT_ID needs to be set"),
            client_secret: std::env::var("OIDC_CLIENT_SECRET")
                .expect("OIDC_CLIENT_SECRET needs to be set"),
            redirect_url: std::env::var("OIDC_REDIRECT_URL")
                .expect("OIDC_REDIRECT_URL needs to be set"),
        })
        .await
        .expect("Error initializing OIDC client"),
    )
}

async fn initialize_youtube() -> Youtube {
    Arc::new(YoutubeClient::new(
        std::env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY needs to be set"),
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // initialize outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    let secret_key = Key::generate();
    let oidc_client = initialize_oidc_client().await;
    let youtube_client = initialize_youtube().await;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .data(youtube_client.clone())
        .finish();

    let app_context = AppContext {
        schema,
        oidc_client,
        youtube_client,
    };

    println!("GraphiQL IDE: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(Data::new(app_context.clone()))
            .service(web::resource("/login").to(auth::login))
            .service(web::resource("/auth_callback").to(auth::auth_callback))
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(index_graphiql),
            )
            .service(web::resource("/").guard(guard::Post()).to(graphql))
            .service(Files::new("/", "public").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

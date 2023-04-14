use crate::handler::graphql_handler::{Mutation, ProjectSchema, Query};
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::r2d2;
use diesel::{prelude::*};
use dotenvy::dotenv;

mod handler;
mod schemas;
mod db_schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

async fn index(context: Data<ProjectSchema>, req: GraphQLRequest) -> GraphQLResponse {
    context.execute(req.into_inner()).await.into()
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
        .expect("database URL should be valid path to SQLite DB file")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // initialize outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish();

    println!("GraphiQL IDE: http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

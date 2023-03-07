mod middleware;
mod models;
mod schema;
mod services;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    http::HeaderMap,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use schema::init_schema;
use schema::StudentsSchema;

use crate::services::new_student_service;

async fn graphql_handler(
    schema: Extension<StudentsSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    req = req.data(middleware::AuthHeaderContainer::extract(&headers));
    schema.execute(req).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphiql")))
}

#[tokio::main]
async fn main() {
    let students_ctx = new_student_service().await;
    let schema = init_schema(students_ctx);

    let app = Router::new()
        .route("/graphiql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8080/graphiql");

    let _ = Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await;
}

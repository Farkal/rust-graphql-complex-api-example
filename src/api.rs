use crate::graphql::{Context, Mutation, Query, Schema};
use actix_web::{web, Error, HttpResponse, Result};
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    ctx: web::Data<Context>,
) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

pub fn playground() -> HttpResponse {
    let html = playground_source("");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn register(config: &mut web::ServiceConfig) {
    let schema = Arc::new(Schema::new(Query, Mutation));

    config
        .data(schema)
        .route("/graphql", web::post().to(graphql))
        .route("/graphql", web::get().to(playground))
}

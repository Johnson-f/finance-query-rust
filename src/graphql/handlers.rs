use actix_web::{web, HttpRequest, HttpResponse, Result};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use crate::graphql::{AppContext, AppSchema};

pub async fn graphql_handler(
    schema: web::Data<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
        )))
}

pub async fn graphql_ws_handler(
    req: HttpRequest,
    body: web::Payload,
    schema: web::Data<AppSchema>,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let ctx = AppContext {
        app_state: app_state.clone(),
    };
    let mut data = async_graphql::Data::default();
    data.insert(ctx);
    GraphQLSubscription::new(schema.get_ref().clone())
        .with_data(data)
        .on_connection_init(|_value| async { 
            Ok::<_, async_graphql::Error>(async_graphql::Data::default())
        })
        .start(&req, body)
}
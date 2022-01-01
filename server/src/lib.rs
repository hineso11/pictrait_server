use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use graphql::AppSchema;
use std::sync::{Arc, Mutex};

#[actix_rt::main]
pub async fn start_server(schema: AppSchema, address: &'static str) -> std::io::Result<()> {
    let counter = Arc::new(Mutex::new(schema));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(counter.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    });

    server.bind(address)?.run().await
}

async fn index(schema: web::Data<Arc<Mutex<AppSchema>>>, req: Request) -> Response {
    let schema = schema.lock().unwrap();

    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let res = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ));

    Ok(res)
}

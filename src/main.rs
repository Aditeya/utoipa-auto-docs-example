mod auth;
mod handler;
mod swagger_docs;
mod types;

use actix_web::{middleware, web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::swagger_docs::ApiDoc;

fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/v1")
        .service(handler::index)
        .service(handler::auth_index)
        .service(handler::create_thing)
        .service(handler::delete_thing);
    conf.service(scope);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .configure(config)
            .service(
                SwaggerUi::new("/docs-v1/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

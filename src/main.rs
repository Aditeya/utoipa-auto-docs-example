mod auth;
mod types;

use crate::{
    auth::AuthToken,
    types::{DeleteRequest, GenericRequest, GenericResponse, PostRequest, PostResponse},
};
use actix_web::{
    delete, get, http::StatusCode, middleware, post, web, App, HttpRequest, HttpResponse,
    HttpServer,
};

#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

#[get("/auth")]
async fn auth_index(_: AuthToken, req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

#[post("/create")]
async fn create_thing(
    req: HttpRequest,
    mut request: web::Json<GenericRequest<(), PostRequest>>,
) -> HttpResponse {
    println!("REQ: {req:?}");

    let name = format!(
        "success: {}",
        request
            .data
            .take()
            .map(|d| d.name)
            .unwrap_or("failed".into())
    );
    let resp = PostResponse { status: name };
    let resp: GenericResponse<PostResponse> = GenericResponse {
        msg: "success".into(),
        data: resp,
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .status(StatusCode::OK)
        .json(resp)
}

#[delete("/delete/{email}")]
async fn delete_thing(
    req: HttpRequest,
    query: web::Query<DeleteRequest>,
    path: web::Path<String>,
) -> HttpResponse {
    println!("REQ: {req:?}");

    let query = query.into_inner();
    let resp: GenericResponse<String> = GenericResponse {
        msg: "Success".into(),
        data: format!(
            "email: {} permanent: {:#?}, when: {:#?}, height: {}",
            path,
            query.permanent.unwrap_or(false),
            query.when.unwrap_or(64),
            query.height
        ),
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .status(StatusCode::OK)
        .json(resp)
}

fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/v1")
        .service(index)
        .service(auth_index)
        .service(create_thing)
        .service(delete_thing);
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web::{delete, get, http::StatusCode, post, web, HttpRequest, HttpResponse};

use crate::{
    auth::AuthToken,
    types::{DeleteRequest, GenericRequest, GenericResponse, PostRequest, PostResponse},
};

/// Root Endpoint
///
/// Hello World Example
#[utoipa::path(
    context_path = "/v1",
    path = "/",
    responses(
        (status = 200, description = "Hello World!")
    )
)]
#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

/// Auth Endpoint
///
/// Basic Auth Example (password is `super-secret-password`)
#[utoipa::path(
    context_path = "/v1",
    responses(
        (status = 200, description = "Hello World!"),
        (status = 401, description = "Invalid")
    ),
    security(
        ("Token" = []),
    )
)]
#[get("/auth")]
async fn auth_index(_: AuthToken, req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

/// Post Endpoint
///
/// Basic Post Example
#[utoipa::path(
    context_path = "/v1",
    responses(
        (status = 200, description = "Hello World!", body = GenericPostResponse),
        (status = 409, description = "Invalid Request Format")
    ),
    request_body = GenericPostRequest,
)]
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

/// Delete Endpoint
///
/// Basic Delete Example
#[utoipa::path(
    context_path = "/v1",
    responses(
        (status = 200, description = "Hello World!", body = GenericStringResponse),
        (status = 409, description = "Invalid Request Format")
    ),
    params(
        ("email" = String, Path, description = "Device UUID"),
        DeleteRequest,
    ),
)]
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

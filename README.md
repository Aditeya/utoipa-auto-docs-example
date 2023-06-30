# Actix Web Auto Docs with Utoipa & Swagger-UI

TODO: add blog post link

## Running
```shell
$ cargo run
```
open `http://localhost:8080/docs-v1/`

## Endpoints
We have the following endpoints setup:
- `GET /v1/` - hello world example
- `GET /v1/auth` - authenticated hello world example
- `POST /v1/create` - basic post request which returns request
- `DELETE /v1/delete/{email}` - basic delete request which returns request

## Swagger UI Images

![SwaggerUI-big](https://github.com/Aditeya/utoipa-auto-docs-example/assets/22963960/e6185937-0930-4976-aeb9-dde946abffad)
Swagger UI `/docs-v1/` homepage

![SwaggerUI-Auth](https://github.com/Aditeya/utoipa-auto-docs-example/assets/22963960/30cb2f73-5275-410e-8d40-b53c899aee15)
Authorisation View to specify custom token

![SwaggerUI-query-path-param](https://github.com/Aditeya/utoipa-auto-docs-example/assets/22963960/5db4e947-c275-4d0f-90fd-595759f452d2)
Query & Path Parameters view

![SwaggerUI-schemas](https://github.com/Aditeya/utoipa-auto-docs-example/assets/22963960/1b02bcc7-484f-49af-88fc-d15412736786)
Schemas Represented in the Endpoint descriptions

![SwaggerUI-request](https://github.com/Aditeya/utoipa-auto-docs-example/assets/22963960/4cb9287e-ea46-45cb-accc-f6a89b657b9e)
Making an example request

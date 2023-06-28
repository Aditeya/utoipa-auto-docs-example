use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[aliases(
    GenericPostResponse = GenericResponse<PostResponse>,
    GenericStringResponse = GenericResponse<String>
)]
pub struct GenericResponse<U> {
    pub msg: String,
    pub data: U,
}

use utoipa::TupleUnit;

#[derive(ToSchema, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[aliases(
    GenericPostRequest = GenericRequest<TupleUnit, PostRequest>,
    GenericDeleteRequest = GenericRequest<TupleUnit, DeleteRequest>,
)]
pub struct GenericRequest<U, V> {
    pub params: Option<U>,
    pub data: Option<V>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PostRequest {
    pub name: String,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PostResponse {
    pub status: String,
}

#[derive(IntoParams, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DeleteRequest {
    /// Delete Permanently ?
    pub permanent: Option<bool>,

    /// when to delete ?
    pub when: Option<u64>,

    /// height of shamlessness (cuz its funny)
    pub height: u64,
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GenericResponse<U> {
    pub msg: String,
    pub data: U,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GenericRequest<U, V> {
    pub params: Option<U>,
    pub data: Option<V>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PostRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PostResponse {
    pub status: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DeleteRequest {
    pub permanent: Option<bool>,
    pub when: Option<u64>,
    pub height: u64,
}

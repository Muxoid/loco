use crate::models::_entities::users;
use axum::{
    http::{HeaderMap, StatusCode},
    response::{AppendHeaders, Redirect},
};
use loco_rs::{
    controller::format::RenderBuilder,
    prelude::{cookie::Cookie, *},
};
use serde::{Deserialize, Serialize};
use serde_json::json; // Make sure to import the `json!` macro

pub fn get_home(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "home/index.html", json!({}))
}

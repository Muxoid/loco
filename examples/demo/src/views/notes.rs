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

pub fn index(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "notes/index.html", json!({}))
}

pub fn post_login(
    v: impl ViewRenderer,
    headers: HeaderMap,
    vue: &str,
) -> Result<impl IntoResponse> {
    let render_builder = format::RenderBuilder::new();

    let hx_redirect = match headers.get("HX-Redirect") {
        Some(value) => value.to_str().unwrap_or_default().to_string(),
        None => "".to_string(),
    };

    let cookie = match headers.get("Set-Cookie") {
        Some(value) => value.to_str().unwrap_or_default().to_string(),
        None => "".to_string(),
    };

    if cookie.is_empty() && hx_redirect.is_empty() {
        println!("Error htmx");
        format::render().view(&v, vue, json!({}))
    } else {
        render_builder
            .header("HX-Redirect", hx_redirect)
            .header("Set-Cookie", cookie)
            .view(&v, vue, json!({}))
    }
}

pub fn get_register(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "auth/register.html", json!({}))
}

pub fn post_register(
    v: impl ViewRenderer,
    headers: HeaderMap,
    vue: &str,
) -> Result<impl IntoResponse> {
    let hx_redirect = match headers.get("HX-Redirect") {
        Some(value) => value.to_str().unwrap_or_default().to_string(),
        None => "".to_string(),
    };

    let render_builder = format::RenderBuilder::new();

    if hx_redirect.is_empty() {
        render_builder.view(&v, vue, json!({}))
    } else {
        render_builder
            .header("HX-Redirect", hx_redirect)
            .view(&v, vue, json!({}))
    }
}

pub fn register_err(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "htmx/err/register_err.html", json!({}))
}

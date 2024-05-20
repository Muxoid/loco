use axum::{
    debug_handler,
    http::{
        header::{HeaderMap, HeaderValue},
        StatusCode,
    },
    response::AppendHeaders,
};
use cookie::Cookie;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    mailers::auth::AuthMailer,
    models::{
        _entities::users,
        users::{LoginParams, RegisterParams},
    },
    views::{self},
}; // Make sure to import the `json!` macro
#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyParams {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

pub async fn render_register(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    views::auth::get_register(v)
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(params): Json<RegisterParams>,
) -> Result<impl IntoResponse> {
    let res = users::Model::create_with_password(&ctx.db, &params).await;

    let mut headers = HeaderMap::new();

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user",
            );
            headers.insert("HX-Redirect", HeaderValue::from_static("/auth/register"));
            return views::auth::post_register(v, headers, "htmx/err/register_err.html");
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;

    headers.insert("HX-Redirect", HeaderValue::from_static("/auth/login"));
    views::auth::post_register(v, headers, "")
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
async fn verify(
    State(ctx): State<AppContext>,
    Json(params): Json<VerifyParams>,
) -> Result<Response> {
    let user = users::Model::find_by_verification_token(&ctx.db, &params.token).await?;

    if user.email_verified_at.is_some() {
        tracing::info!(pid = user.pid.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(&ctx.db).await?;
        tracing::info!(pid = user.pid.to_string(), "user verified");
    }

    format::json(())
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return format::json(());
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    AuthMailer::forgot_password(&ctx, &user).await?;

    format::json(())
}

/// reset user password by the given parameters
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    let Ok(user) = users::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    format::json(())
}

pub async fn render_login(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    views::auth::get_login(v)
}

/// Creates a user login and returns a token
async fn login(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Json(params): Json<LoginParams>,
) -> Result<impl IntoResponse> {
    let get_user = users::Model::find_by_email(&ctx.db, &params.email).await;

    let mut headers = HeaderMap::new();
    let user = match get_user {
        Ok(user) => user,
        Err(_) => {
            // Log the error or handle it appropriately here
            return views::auth::post_login(v, headers, "htmx/err/login_err.html");
        }
    };

    let valid = user.verify_password(&params.password);

    if !valid {
        return views::auth::post_login(v, headers, "htmx/err/login_err.html");
    }

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    headers.insert("HX-Redirect", HeaderValue::from_static("/home"));

    let cookie_value = format!(
        "{}={}; HttpOnly; Path=/",
        "loco_cookie_key".to_string(),
        token
    );
    headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

    views::auth::post_login(v, headers, "auth/login.html")
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("auth")
        .add("/register", post(register).get(render_register))
        .add("/verify", post(verify))
        .add("/login", post(login).get(render_login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
}

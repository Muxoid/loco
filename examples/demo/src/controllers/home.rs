#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::{
    mailers::auth::AuthMailer,
    models::{
        _entities::users,
        users::{LoginParams, RegisterParams},
    },
    views::{self},
};

pub async fn index(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    // do something with context (database, etc)
    views::home::get_home(v)
}

pub fn routes() -> Routes {
    Routes::new().prefix("home").add("/", get(index))
}

use std::{collections::HashMap, sync::Once};

use axum::http::header;
use axum::http::StatusCode;
use axum::{body::Body, extract::Path, http::Response, Router};

use super::{
    err::{self, ResponseError},
    Launcher,
};

pub(super) mod arkose;
pub(super) mod ui;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

static INIT: Once = Once::new();
static mut STATIC_FILES: Option<HashMap<&'static str, static_files::Resource>> = None;

pub(super) fn config(router: Router, args: &Launcher) -> Router {
    INIT.call_once(|| unsafe { STATIC_FILES = Some(generate()) });
    let router = ui::config(router, args);
    let router = arkose::config(router, args);
    router
}

async fn get_static_resource(path: Path<String>) -> Result<Response<Body>, ResponseError> {
    let path = path.0;
    let mut x = unsafe { STATIC_FILES.as_ref().unwrap().iter() };
    match x.find(|(k, _v)| k.contains(&path)) {
        Some((_, v)) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, v.mime_type)
            .body(Body::from(v.data))
            .map_err(|err| err::ResponseError::InternalServerError(err))?),
        None => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .map_err(|err| err::ResponseError::InternalServerError(err))?),
    }
}

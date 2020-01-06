use hyper::{Response, Body, StatusCode};

pub const INDEX: &[u8] = include_bytes!("../../hoo_frontend/dist/index.html");
pub const PACKAGE_JS: &[u8] = include_bytes!("../../hoo_frontend/dist/pkg/package.js");
pub const PACKAGE_WASM: &[u8] = include_bytes!("../../hoo_frontend/dist/pkg/package_bg.wasm");
pub const BUNDLE: &[u8] = include_bytes!("../../hoo_frontend/dist/pkg/bundle.js");

pub fn index() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(INDEX.into())
        .unwrap()
}

pub fn package_js() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/javascript")
        .body(PACKAGE_JS.into())
        .unwrap()
}

pub fn package_wasm() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/wasm")
        .body(PACKAGE_WASM.into())
        .unwrap()
}

pub fn bundle() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/javascript")
        .body(BUNDLE.into())
        .unwrap()
}

use crate::cli::http_opts::ServerOpts;
use anyhow;
use tracing::{info, warn};
use axum::{
    extract::{path, Path, State}, http::StatusCode, routing::get, Router
};
use std::path::PathBuf;

use tokio::net::TcpListener;
use std::sync::Arc;
#[derive(Debug)]
struct httpServerState {
    path: PathBuf,
}

pub async fn process_http_server(opts: &ServerOpts) -> anyhow::Result<()> {
    let state = httpServerState {
        path: opts.path.clone(),
    };
    let router = Router::new()
        .route("/{*key}", get
        (index_handler))
        .with_state(Arc::new(state));

        info!("http server start at {}:{}", "127.0.0.1", opts.port);
        info!("http server root dir: {:?}", opts.path);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", opts.port)).await.unwrap();
    axum::serve(listener, router).await.unwrap();


    Ok(())
}

async fn index_handler(State(state): State<Arc<httpServerState>>, Path(path): Path<String>) -> (StatusCode, String) {
    // "Hello, World!"
   
    let p = state.path.join(path);
    if ! p.exists() {
        return (StatusCode::NOT_FOUND, format!("http server root dir: {:?}, path: {} not exist", state.path, p.display()));
    }else{
      let ret =  match tokio::fs::read_to_string(p).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => {
                warn!("http server read file error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("http server read file error: {:?}", e))
            }
        };
        ret
    }
}
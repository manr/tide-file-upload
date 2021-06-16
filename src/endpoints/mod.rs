use async_std::{fs::File, io};
use tide::{Body, Request, Response, StatusCode};

use crate::types::{AppState, FileUploadResponse, IndexResponse};

pub async fn get_index(mut _req: Request<AppState>) -> tide::Result {
    let response = IndexResponse {
        value: "Welcome".into(),
    };
    if let Ok(body) = Body::from_json(&response) {
        Ok(body.into())
    } else {
        Err(tide::Error::from_str(403, "bad"))
    }
}

pub async fn get_file(req: Request<AppState>) -> tide::Result {
    let path = req.param("file")?;
    let fs_path = req.state().path().join(path);

    if let Ok(body) = Body::from_file(&fs_path).await {
        Ok(body.into())
    } else {
        tide::log::error!("file not found", {
            path: fs_path.to_str(),
        });
        Ok(Response::new(StatusCode::NotFound))
    }
}

pub async fn put_file(req: Request<AppState>) -> tide::Result {
    let path = req.param("file")?;
    let fs_path = req.state().path().join(path);
    let f = File::create(&fs_path).await?;
    let bytes_written = io::copy(req, f).await?;
    let path_buf = fs_path.canonicalize()?;
    let path_str = match path_buf.to_str() {
        Some(s) => s,
        None => "",
    };

    tide::log::info!("file written", {
        bytes: bytes_written,
        path: path_str
    });

    let response = FileUploadResponse {
        name: String::from(path_str),
        size: bytes_written,
    };
    if let Ok(body) = Body::from_json(&response) {
        Ok(body.into())
    } else {
        Ok(Response::new(StatusCode::BadRequest))
    }
}

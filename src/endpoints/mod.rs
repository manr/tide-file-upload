use async_std::fs;
use async_std::{fs::File};
use async_std::io::{BufReader, BufWriter, prelude::*};
use tide::{Body, Request, Response, StatusCode};

use crate::types::{AppState, FileUploadResponse, IndexResponse};

const FILE_LIMIT: u64 = 1024 ^ 3;

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

/*
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
 */

pub async fn put_file_limited(req: Request<AppState>) -> tide::Result {
    let path = req.param("file")?;
    let fs_path = req.state().path().join(path);
    let f = File::create(&fs_path).await?;
    
    let mut buf_reader = BufReader::new(req);
    let mut buf_writer = BufWriter::new(f);
    let mut buf = vec![0u8; 1024 * 1024];
    let mut bytes_written: u64 = 0;

    loop {
        let bytes_read = buf_reader.read(&mut buf).await?;
        if bytes_read > 0 {
            tide::log::info!("bytes read", {
                bytes: &bytes_read,
            });
            buf_writer.write(&buf[0..bytes_read]).await?;
            bytes_written = bytes_written + bytes_read as u64;

            if bytes_written > FILE_LIMIT {
                tide::log::warn!("file limit exceeded"); 

                fs::remove_file(fs_path).await?;
                
                return Ok(Response::new(StatusCode::BadRequest))
            }
        } else {
            if bytes_written > 0 {
                buf_writer.flush().await?;
            }
            break;
        }
    }

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

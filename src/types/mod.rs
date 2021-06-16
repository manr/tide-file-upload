use std::{path::Path, sync::Arc};

use tempfile::TempDir;
use tide::prelude::*;

#[derive(Clone)]
pub struct AppState {
    tempdir: Arc<TempDir>,
}

impl AppState {
    pub(crate) fn new() -> anyhow::Result<AppState> {
        let tmp = TempDir::new_in("./")?;
        let tmp_dir = Arc::new(tmp);
        Ok(AppState { tempdir: tmp_dir })
    }

    pub(crate) fn path(&self) -> &Path {
        self.tempdir.path()
    }
}

#[derive(Debug, Serialize)]
pub struct IndexResponse {
    pub(crate) value: String,
}

#[derive(Debug, Deserialize)]
pub struct FileUploadRequest {
    pub(crate) name: String,
    pub(crate) size: u64,
}

#[derive(Debug, Serialize)]
pub struct FileUploadResponse {
    pub(crate) name: String,
    pub(crate) size: u64,
}

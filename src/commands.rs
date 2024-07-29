use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::StructureManagerExt;

#[command]
#[allow(dead_code)]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.structure_manager().ping(payload)
}

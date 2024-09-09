use idl_gen::clipboard::*;
use volo::FastStr;
use volo_grpc::*;

use crate::cfg::get_global_cfg;
pub struct ClipboardService;

impl idl_gen::clipboard::Clipboard for ClipboardService {
    async fn send(
        &self,
        req: Request<SendClipboardRequest>,
    ) -> Result<Response<SendClipboardResponse>, Status> {
        todo!()
    }
}

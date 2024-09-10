use idl_gen::{base::BaseResp, clipboard::*};
use normalize_path::NormalizePath as _;
use tokio::io::AsyncWriteExt as _;
use volo::FastStr;
use volo_grpc::*;

use crate::cfg::get_global_cfg;
pub struct ClipboardService;

impl idl_gen::clipboard::Clipboard for ClipboardService {
    async fn send(
        &self,
        req: Request<SendClipboardRequest>,
    ) -> Result<Response<SendClipboardResponse>, Status> {
        let cfg = get_global_cfg();
        let project_root = cfg.project_root;
        match cfg.clipboard_proxy {
            Some(cfg) => {
                if !cfg.enable {
                    return new_disable_resp();
                }
                let path = &req.get_ref().file_path;
                if path.is_empty() {
                    return Err(Status::new(Code::Internal, "file path is empty"));
                }
                let path: std::path::PathBuf = path.as_str().parse().map_err(|_| {
                    Status::new(Code::Internal, format!("path parse failed: {}", path))
                })?;
                let path = path.as_path().normalize();
                if !path.is_absolute() || !path.starts_with(project_root) {
                    return Err(Status::new(
                        Code::InvalidArgument,
                        "file path is not in project root or not absolute",
                    ));
                }
                let s = tokio::fs::read_to_string(path).await?;
                let addr = cfg.forward_host.into();
                let socket = tokio::net::TcpSocket::new_v4()?;
                let mut stream = socket.connect(addr).await?;
                stream.write_all(&s.len().to_be_bytes()).await?;
                stream
                    .write_all(&crc32fast::hash(s.as_bytes()).to_be_bytes())
                    .await?;
                stream.write_all(s.as_bytes()).await?;
                drop(stream);
                Ok(Response::new(SendClipboardResponse {
                    base_resp: Some(BaseResp {
                        code: 0,
                        msg: FastStr::new(s),
                        trace_id: FastStr::default(),
                    }),
                }))
            }
            None => new_disable_resp(),
        }
    }
}

fn new_disable_resp() -> Result<Response<SendClipboardResponse>, Status> {
    Err(Status::new(
        Code::InvalidArgument,
        "you dont enable clipboard proxy",
    ))
}

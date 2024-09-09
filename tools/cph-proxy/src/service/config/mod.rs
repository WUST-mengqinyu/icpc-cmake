use idl_gen::info::*;
use volo::FastStr;
use volo_grpc::*;
mod clipboard;
pub use clipboard::*;

use crate::cfg::get_global_cfg;
pub struct EchoService;

impl idl_gen::info::Echo for EchoService {
    async fn echo(&self, req: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let resp = EchoResponse {
            message: format!("helloworld, {}!", req.get_ref().message).into(),
            ..Default::default()
        };
        Ok(Response::new(resp))
    }
}

impl idl_gen::info::ConfigInfo for EchoService {
    async fn local_config(
        &self,
        _req: Request<LocalConfigRequest>,
    ) -> Result<Response<LocalConfigResponse>, Status> {
        let local_cfg = get_global_cfg();
        let running_mode = local_cfg.running_mode.map(|mode| RunningMode {
            enable: mode.enable,
            running_path: FastStr::new(mode.running_path.display().to_string()),
            remove_old_linkers: mode.remove_old_linkers,
        });
        Ok(Response::new(LocalConfigResponse {
            config: Some(Config {
                listen_host: Some(local_cfg.listen_host.into()),
                to_host: local_cfg.to_host.map(|x| x.into()),
                to_host_proxy: local_cfg.to_host_proxy.map(|x| x.into()),
                codeforces_project_path: FastStr::new(
                    local_cfg.codeforces_project_path.display().to_string(),
                ),
                atcoder_project_path: FastStr::new(
                    local_cfg.atcoder_project_path.display().to_string(),
                ),
                others_project_path: FastStr::new(
                    local_cfg.others_project_path.display().to_string(),
                ),
                lock_file_delay_seconds: local_cfg.lock_file_delay_seconds,
                lock_file_max_try: local_cfg.lock_file_max_try,
                must_self_host: local_cfg.must_self_host,
                running_mode,
                setted_by: local_cfg.setted_by.into(),
            }),
            ..Default::default()
        }))
    }
}

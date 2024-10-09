pub mod common_utils;
pub mod vscode_middleware;

use crate::model::*;
use crate::service::ProblemMetaWithTestCaseHandler;

pub struct DefaultHandler {}

#[async_trait::async_trait]
impl ProblemMetaWithTestCaseHandler for DefaultHandler {
    fn detecte(&self, _data: &ProblemMetaWithTestCase) -> anyhow::Result<String> {
        todo!()
    }
    async fn handle(&self, _data: &ProblemMetaWithTestCase) -> anyhow::Result<()> {
        todo!()
    }
}

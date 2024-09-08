pub mod common_utils;

use crate::service::ProblemMetaWithTestCaseHandler;
use crate::model::*;

pub struct DefaultHandler {}

#[async_trait::async_trait]
impl ProblemMetaWithTestCaseHandler for DefaultHandler {
    async fn handle(&self, _data: &ProblemMetaWithTestCase) -> anyhow::Result<()> {
        todo!()
    }
}

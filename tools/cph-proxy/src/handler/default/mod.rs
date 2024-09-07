pub mod common_utils;

use crate::handler::ProblemMetaWithTestCaseHandler;
use crate::model::*;

pub struct DefaultHandler {}
impl ProblemMetaWithTestCaseHandler for DefaultHandler {
    fn handle(&self, _data: &ProblemMetaWithTestCase) -> anyhow::Result<()> {
        todo!()
    }
}

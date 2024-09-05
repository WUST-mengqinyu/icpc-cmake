use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProblemMetaWithTestCase {
    pub name: String,
    pub group: String,
    pub url: String,
    pub interactive: bool,
    #[serde(alias = "memoryLimit")]
    pub memory_limit: u32,
    #[serde(alias = "timeLimit")]
    pub time_limit: u32,
    pub tests: Vec<TestCase>,
    #[serde(alias = "testType")]
    pub test_type: TestType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TestType {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "multiNumber")]
    MultiNumber,
    Unknown(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

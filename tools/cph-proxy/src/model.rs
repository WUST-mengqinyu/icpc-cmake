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
    pub batch: Batch,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Batch {
    pub id: String,
    pub size: u32,
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

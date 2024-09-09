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

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn mock_problem_meta_with_test_case() -> ProblemMetaWithTestCase {
        serde_json::from_str(
            "{
    \"name\": \"G. Castle Defense\",
    \"group\": \"Codeforces - Educational Codeforces Round 40 (Rated for Div. 2)\",
    \"url\": \"https://codeforces.com/problemset/problem/954/G\",
    \"interactive\": false,
    \"memoryLimit\": 256,
    \"timeLimit\": 1500,
    \"tests\": [
        {
            \"input\": \"5 0 6\\n5 4 3 4 9\\n\",
            \"output\": \"5\\n\"
        },
        {
            \"input\": \"4 2 0\\n1 2 3 4\\n\",
            \"output\": \"6\\n\"
        },
        {
            \"input\": \"5 1 1\\n2 1 2 1 2\\n\",
            \"output\": \"3\\n\"
        }
    ],
    \"testType\": \"single\",
    \"input\": {
        \"type\": \"stdin\"
    },
    \"output\": {
        \"type\": \"stdout\"
    },
    \"languages\": {
        \"java\": {
            \"mainClass\": \"Main\",
            \"taskClass\": \"GCastleDefense\"
        }
    },
    \"batch\": {
        \"id\": \"123e67c8-03c6-44a4-a3f9-5918533f9fb2\",
        \"size\": 1
    }
}",
        )
        .expect("gen mock problem metadata with test case failed")
    }
}

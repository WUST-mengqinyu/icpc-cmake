// todo: make it in text template
// todo: support in cfg

const DEFAULT_CODEFORCES_CMAKE_LIST_TEMPLATE: &str =
    "add_executable(cf_{contest_id}_{problem_id} test.cc)
target_link_libraries(cf_{contest_id}_{problem_id} GTest::gtest_main)
include(GoogleTest)
include_directories(${PROJECT_DIR})
gtest_discover_tests(cf_{contest_id}_{problem_id})
";

const DEFAULT_TEST_CC_TEMPLATE: &str = "#include \"inner/tools/test_suite.hpp\"
#include \"main.h\"
#include <cstring>
#include <gtest/gtest.h>

const std::string CUR_DIR = \"{cur_dir}/cases/\";

// {start_loop}
TEST_SAMPLE_CASE({i})
// {end_loop}
";

const DEFAULT_MAIN_HEADER_TEMPLATE: &str = 
"#include \"inner/prelude\"
using namespace inner;

MAIN() {
  return 0;
}
";

use super::context;
use crate::model::ProblemMetaWithTestCase;
use anyhow::{anyhow, Result};
use std::{collections::HashMap, io::Write, path::Path};

pub fn cmake_gen(cx: context::CodeforcesContext, data: &ProblemMetaWithTestCase) -> Result<()> {
    let cmake_list = DEFAULT_CODEFORCES_CMAKE_LIST_TEMPLATE
        .replace("{contest_id}", cx.contest_id.to_string().as_str())
        .replace("{problem_id}", &cx.problem_id);

    let test_cc = DEFAULT_TEST_CC_TEMPLATE
        .replace(
            "{cur_dir}",
            cx.home_dir
                .as_path()
                .as_os_str()
                .to_str()
                .ok_or(anyhow!("home dir to string failed: {:?}", cx.home_dir))?,
        )
        .replace_loop(
            data.tests
                .len()
                .try_into()
                .map_err(|e| anyhow!("too many test case: {}!, e: {}", data.tests.len(), e))?,
        );
    let main_header = DEFAULT_MAIN_HEADER_TEMPLATE.to_owned();

    create_if_not_exist(&HashMap::from([
        (cx.home_dir.join("CMakeLists.txt").as_path(), cmake_list),
        (cx.home_dir.join("test.cc").as_path(), test_cc),
        (cx.home_dir.join("main.h").as_path(), main_header),
    ]))?;

    let codeforces_parent_cmake_path = cx
        .home_dir
        .parent() // contest
        .and_then(|p| p.parent()) // codeforces root
        .ok_or(anyhow!(
            "codeforces home is not found [{:?}/..]?",
            cx.home_dir
        ))?
        .join("CMakeLists.txt");
    let codeforces_parent_cmake = std::fs::read_to_string(&codeforces_parent_cmake_path)?;
    let append_line = &format!("add_subdirectory({}/{})", cx.contest_id, cx.problem_id);
    if !codeforces_parent_cmake.contains(append_line) {
        let mut f = std::fs::OpenOptions::new()
            .create(false)
            .append(true)
            .open(&codeforces_parent_cmake_path)?;
        if codeforces_parent_cmake
            .as_bytes()
            .last()
            .is_some_and(|c| c.ne(&b'\n'))
        {
            f.write_all(&[b'\n'])?;
        }
        f.write_all(append_line.as_bytes())?;
        f.write_all(&[b'\n'])?;
        f.sync_all()?;
    }
    Ok(())
}

trait LoopReplace {
    fn replace_loop(self, n: u32) -> String;
}

impl LoopReplace for &str {
    fn replace_loop(self, n: u32) -> String {
        if let Some(start_pos) = self.find("{start_loop}") {
            if let Some(end_pos) = self[start_pos..].find("{end_loop}") {
                let end_pos = start_pos + end_pos;
                let res_m = &self[start_pos + "{start_loop}".len()..=end_pos];
                let mut res = self[..=start_pos].to_owned();
                for i in 0..n {
                    res.push_str(res_m.replace("{i}", i.to_string().as_str()).as_str());
                }
                res.push_str(&self[end_pos + ("{end_loop}".len()) + 1..]);
                return res;
            }
        }
        self.to_owned()
    }
}

fn create_if_not_exist(mp: &HashMap<&Path, String>) -> Result<()> {
    for (path, content) in mp {
        let f = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .append(false)
            .open(path);
        if f.as_ref()
            .is_err_and(|e| e.kind() == std::io::ErrorKind::AlreadyExists)
        {
            continue;
        }
        let mut f = f?;
        f.write_all(content.as_bytes())?;
        f.sync_all()?;
    }
    Ok(())
}

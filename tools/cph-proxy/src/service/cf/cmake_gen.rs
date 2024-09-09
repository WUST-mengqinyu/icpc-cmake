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

const std::string CUR_DIR = PROJECT_DIR + \"/archive/codeforces/{contest_id}/{problem_id}/cases/\";

{start_loop}TEST_SAMPLE_CASE({i})
{end_loop}
";

const DEFAULT_MAIN_HEADER_TEMPLATE: &str = "#include \"inner/prelude\"
using namespace inner;

MAIN() {
  return 0;
}
";

use super::CodeforcesHandler;
use crate::model::ProblemMetaWithTestCase;
use crate::service::{create_files_if_absent, recreated_ref_in_running, LoopReplace};
use anyhow::{anyhow, Result};

impl CodeforcesHandler {
    pub(super) async fn cmake_gen(
        cx: super::CodeforcesContext,
        data: &ProblemMetaWithTestCase,
    ) -> Result<()> {
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
            .replace("{contest_id}", cx.contest_id.to_string().as_str())
            .replace("{problem_id}", &cx.problem_id)
            .replace_loop(
                data.tests
                    .len()
                    .try_into()
                    .map_err(|e| anyhow!("too many test case: {}!, e: {}", data.tests.len(), e))?,
            );
        let main_header = DEFAULT_MAIN_HEADER_TEMPLATE.to_owned();

        create_files_if_absent(&[
            (cx.home_dir.join("CMakeLists.txt").as_path(), cmake_list),
            (cx.home_dir.join("test.cc").as_path(), test_cc),
            (cx.home_dir.join("main.h").as_path(), main_header),
        ])
        .await?;

        recreated_ref_in_running(
            &data.batch.id,
            cx.home_dir.join("main.h").as_path(),
            &format!("{}.cc", cx.problem_id),
        )
        .await
    }
}

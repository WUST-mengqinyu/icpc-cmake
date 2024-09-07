#include "color.hpp"
#include "inner/tools/wcmp.hpp"
#include <gtest/gtest.h>
#include <string>

// FIXME: output and ans path
template<class F, class CMP>
void test_io(const std::string testCase, const std::string &inputPath, const std::string &outputPath, const std::string &ansPath, F solve, CMP cmp) {
  pid_t pid = fork();
  if (pid == 0) {
    freopen(inputPath.c_str(), "r", stdin);
    freopen(ansPath.c_str(), "w", stdout);
    solve();
    freopen("/dev/tty", "a", stdout);
    _exit(0);
  } else {
    int status;
    clock_t ST = clock();
    waitpid(pid, &status, 0);
    std::cout << ColorS(color_orange, color_dark_blue, "Case #" + testCase) << " solve use time: " << ((clock() - ST) * 1000.0 / CLOCKS_PER_SEC) << "ms" << std::endl;
    bool equal = wcmp(ansPath, outputPath);
    if (!equal) {
      std::cout << ColorS(color_red, color_dark_blue, "case input:") << " " << inputPath << std::endl;
    }
    if (status != 0) {
      std::cerr << RE << " " << "case input: " << inputPath << " exit:" << status << std::endl;
    }
    EXPECT_EQ(status, 0);
    EXPECT_TRUE(equal);
  }
}


template<class F, class CMP>
void test_io_with_spec(const std::string &workDir, const std::string &testCase, F solve, CMP cmp) {
  auto inputPath = workDir + testCase + ".in";
  auto outputPath = workDir + testCase + ".out";
  auto ansPath = workDir + testCase + ".ans";

  test_io(testCase, inputPath, outputPath, ansPath, solve, cmp);
}

#define TEST_SAMPLE_CASE(case_name)                      \
  TEST(SAMPLE_SUITE, case_##case_name) {                 \
    test_io_with_spec(CUR_DIR, #case_name, solve, wcmp); \
  }

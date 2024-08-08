#include "inner/ds/seg_tree_op.hpp"
#include <gtest/gtest.h>
#include <print>

TEST(SegTreeOpTest, BasicAssertions) {
  using namespace inner::seg_tree_op;
  seg<int, int> sg(5);
  sg.set(1, 5);
  sg.set(2, 3);
  sg.apply(1, 3, 2);
  sg.set(2, 1);
  sg.apply(2, 7);
  EXPECT_EQ(5 + 8 + 2 * 2, sg.sum(1, 5));
}
#include "inner/ds/seg_tree_op.hpp"
#include "inner/num/bit.hpp"
#include <gtest/gtest.h>
#include <print>

TEST(SegTreeOpTest, BasicAssertions) {
  using namespace inner::seg_tree_op;
  seg<int, int> sg(5);
  sg.set(1, 5);
  sg.set(2, 3);
  sg.apply(1, 3, 2);
  sg.set<[](int &x) { x = 1; }>(2);
  sg.apply(2, 7);
  EXPECT_EQ(5 + 8 + 2 * 2, sg.sum(1, 5));
  seg<int, NullType> sg2(10);
  sg2.set(1, 5);
  sg2.set(3, 7);
  EXPECT_EQ(5, sg2.get(1));
  EXPECT_EQ(5 + 7, sg2.sum(1, 5));
}

TEST(Bit, BasicAssertions) {
  using namespace inner::num_bit;
  EXPECT_EQ(3, ceil_pow2(5));
}
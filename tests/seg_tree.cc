#include "inner/ds/seg_tree_op.hpp"
#include "inner/ds/seg_tree_poly_node.hpp"
#include <gtest/gtest.h>
using namespace inner;

TEST(SegTreeOpTest, BasicAssertions) {
  seg<int, NullTag> sg2(10);
  sg2.set(1, 5);
  sg2.set(3, 7);
  EXPECT_EQ(5, sg2.get(1));
  EXPECT_EQ(5 + 0 + 7 + 0 + 0, sg2.sum(1, 5));
};

TEST(SegTreeOpTest, PolyMaxAssertions) {
  using tg = AddTag<int>;
  using v = PolyNode<int, NodeType::Add>;
  auto sg = seg<v, tg>(std::vector(10, v(0)));
  sg.apply(1, 5, AddTag(1));
  EXPECT_EQ(1 + 1 + 1 + 1 + 1, sg.sum(1, 5).val());
};

#include "inner/ds/seg_tree_poly_node.hpp"
#include <cstdio>
#include <gtest/gtest.h>
#include <utility>
using namespace inner;

TEST(PolyStruct, NodeAssertions) {
  auto x = PolyTag<int, NodeType::Add>(5);
  auto y = PolyTag<int, NodeType::Set>(7);
  auto z = PolyTag<int, NodeType::Set>(3);
  x.apply(y);
  x.apply(z);
  EXPECT_EQ(3, x.val());
  x.apply(PolyTag<int, NodeType::Max>(7));
  x.apply(PolyTag<int, NodeType::Max>(3));
  EXPECT_EQ(7, x.val());
}

TEST(PolyStruct, PolyAssertions) {
  auto x = Poly(MaxTag(5), MaxTag(3));
  auto y = Poly(MaxTag(6), MaxTag(8));
  x.value_add(y);
  EXPECT_EQ(8, std::get<0>(x.x).val());
  EXPECT_EQ(8, std::get<1>(x.x).val());
}

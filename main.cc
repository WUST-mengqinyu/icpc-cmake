#include "inner/ds/seg_tree_op.hpp"
#include "inner/num/static_mod_int.hpp"
#include "inner/prelude"

#define VI vector<int>
#define ll long long
using namespace std;

const int maxn = 2e5 + 50;
const int mod = 1e9 + 7;

int f[maxn][10];
using mint = inner::static_mod_int::modint998244353;

using namespace inner::seg_tree_op;

template<class T>
struct PolyTag {
  vector<T> poly;
};

enum PolyOpType {
  Add,
  Set,
  Mul,
};

template<class T>
struct PolyOp {
  PolyOpType tp;
  int p;
  T v;
};


int main(int argc, char **agrv) {
  // seg<mint, PolyTag<mint>> sg;
  // sg.apply(1, 2, PolyOp<mint>{

  //                });
  return 0;
}

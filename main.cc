#include "inner/number"
#include "inner/prelude"

#define VI vector<int>
#define ll long long
using namespace std;

const int maxn = 2e5 + 50;
const int mod = 1e9 + 7;

int f[maxn][10];

int main(int argc, char **agrv) {
  inner::num::init(100);
  W(inner::num::factor(10));
  TKASE {
    int a;
    R(a);
    W(NoSuffix("case #"), kase, inner::qp<mod>(22, a));
  }
  return 0;
}

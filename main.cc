#include "atcoder/all"
#include "inner/prelude"

#define VI vector<int>
#define ll long long
using namespace std;

const int maxn = 2e5 + 50;
const int mod = 1e9 + 7;

int f[maxn][10];
typedef atcoder::modint998244353 mint;
int main(int argc, char **agrv) {
  mint a = 10;
  mint b = 1000;
  W((b / a).val(), (a / b).val(), a.pow(100).val());
  TKASE {
    int a;
    R(a);
    W(NoSuffix("case #"), kase, inner::qp<mod>(22, a));
  }
  return 0;
}

/*================================================================
*
*   创 建 者： badcw
*   创建日期： 2024-07-19 23:27:36
*
================================================================*/
#include "inner/prelude"

#define VI vector<int>
#define ll long long
using namespace std;

const int maxn = 2e5 + 50;
const int mod = 1e9 + 7;

int f[maxn][10];

int main(int argc, char **agrv) {
  TKASE {
    int a;
    R(a);
    W(NoSuffix{"case #"}, kase, inner::qp<mod>(2, a));
  }
  return 0;
}

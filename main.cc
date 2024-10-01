#include "inner/prelude"
#include <algorithm>
#include <cstdio>
using namespace std;
const int N = 200003;
const long long PIN = 4557430888798830399;
struct Line {
  int a, b, c;// y = -(a / b) * x + (c / b)
} a[N], b[N];
const Line era = {0, 1, 0};
int n, m;
long long c[N];
bool cmp(const Line &l1, const Line &l2) {
  return (long long) l1.a * l2.b < (long long) l1.b * l2.a;
}
long long sec(Line l1, Line l2) {
  // It is guaranteed that a1 / b1 <= a2 / b2.
  if ((long long) l1.c * l2.b > (long long) l1.b * l2.c) return -1;
  if ((long long) l1.a * l2.b == (long long) l1.b * l2.a) return PIN;
  long long sa = (long long) l1.b * l2.a - (long long) l1.a * l2.b;
  long long sb = (long long) l1.b * l2.c - (long long) l1.c * l2.b;
  return sb / sa;
}
int floor(int a, int b) {
  return (a >= 0) ? (a / b) : -((-a + b - 1) / b);
}
long long f(int a, int b, int c, int n) {
  // Calculate the sum of (a * i + b) / c(1 <= i <= n).
  long long sa = floor(a, c), sb = floor(b, c);
  long long ans = sa * n * (n + 1) / 2 + sb * n;
  a -= sa * c, b -= sb * c;
  if (a == 0 || n == 0) return ans;
  return ((long long) a * n + b) / c * n - f(c, -b - 1, a, ((long long) a * n + b) / c) + ans;
}
int main() {
  //	freopen("line.in","r",stdin);
  //	freopen("line.out","w",stdout);
  int i, t;
  long long ans;
  for (scanf("%d", &t); t > 0; t--) {
    scanf("%d", &n), m = 0;
    for (i = 1; i <= n; i++)
      scanf("%d%d%d", &a[i].a, &a[i].b, &a[i].c), a[i].c--;
    sort(a + 1, a + n + 1, cmp);
    for (i = 1; i <= n; i++) {
      while (m > 0 && sec(b[m], a[i]) == -1) m--;
      while (m > 1 && sec(b[m], a[i]) <= sec(b[m - 1], b[m])) m--;
      b[++m] = a[i];
    }
    while (m > 1 && sec(era, b[m - 1]) <= sec(b[m - 1], b[m])) m--;

    for (int i = 0; i <= m; ++i) {
      DBGLN(b[i].a << " " << b[i].b << " " << b[i].c);
      //   auto [a, b, c] = in[i];
      //   W("Segment", geo::db(c) / a, 0, 0, geo::db(c) / b);
    }

    for (i = 1; i < m; i++)
      c[i] = sec(b[i], b[i + 1]);
    c[m] = sec(era, b[m]), ans = 0;
    for (i = 1; i <= m; i++) {
      W("*", c[i - 1], c[i], b[i].a, b[i].b, b[i].c);
      ans += f(-b[i].a, b[i].c, b[i].b, c[i]);
      ans -= f(-b[i].a, b[i].c, b[i].b, c[i - 1]);
    }
    printf("%lld\n", ans);
  }
  //	fclose(stdin);
  //	fclose(stdout);
  return 0;
}
#include "inner/prelude"
using namespace inner;

MAIN() {
  int n;
  R(n);
  VI a(n);
  R(a);
  ll f = 0, g = 0;
  f = a[0];
  for (int i = 1; i < n; ++i) {
    ll fn = std::max(f, g + a[i]);
    ll gn = std::max(g, f + a[i] * 2);
    f = fn;
    g = gn;
  }
  W(std::max(f, g));
  return 0;
}

// 1 5+5 3 7+7

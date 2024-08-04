#include "inner/prelude"

namespace inner {
  namespace inv_gcd {
    constexpr std::pair<ll, ll> inv_gcd(ll a, ll b) {
      a = safe_mod(a, b);
      if (a == 0) return {b, 0};
      ll s = b, t = a;
      ll m0 = 0, m1 = 1;
      while (t) {
        ll u = s / t;
        s -= t * u;
        m0 -= m1 * u;
        auto tmp = s;
        s = t;
        t = tmp;
        tmp = m0;
        m0 = m1;
        m1 = tmp;
      }
      if (m0 < 0) m0 += b / s;
      return {s, m0};
    }

  }// namespace inv_gcd
}// namespace inner

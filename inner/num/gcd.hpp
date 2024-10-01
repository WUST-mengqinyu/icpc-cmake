#include "inner/prelude"

namespace inner {
  namespace gcd {
    template<class T>
    constexpr T gcd(T a, T b) {
      auto z = T(0);
      while (b != z) {
        T t = a % b;
        a = b;
        b = t;
      }
      return a;
    }

    template<class T>
    constexpr T lcm(T a, T b) {
      return a / gcd(a, b) * b;
    }
  }// namespace gcd
}// namespace inner
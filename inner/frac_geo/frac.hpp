#ifndef GEO_DB
#define GEO_DB

#include "inner/num/gcd.hpp"
#include "inner/prelude"// IWYU pragma: keep
namespace inner {
  namespace frac_geo {

    template<class T = ll>
    struct p2 {
      T p, q;// p / q
      p2(T _p = 0, T _q = 1, bool auto_r = false) {
        if (auto_r) reduction(_p, _q);
        p = _p, q = _q;
      }
      void reduction(T &a, T &b) {
        if (a < T(0) && b < T(0)) a = -a, b = -b;
        if (a == T(0)) {
          b = T(1);
          return;
        }
        T tmp = ::inner::gcd::gcd(a, b);
        a /= tmp, b /= tmp;
      }
      fr p2 operator+(p2 x, p2 y) { return p2(x.p * y.q + x.q * y.p, x.q * y.q); }
      fr p2 operator-(p2 x, p2 y) { return p2(x.p * y.q - x.q * y.p, x.q * y.q); }
      fr p2 operator*(p2 x, p2 y) { return p2(x.p * y.p, x.q * y.q); }
      fr p2 operator/(p2 x, p2 y) { return p2(x.p * y.q, x.q * y.p); }
      fr p2 operator-(p2 x) { return p2(-x.p, x.q); }
      fr bool operator<(p2 x, p2 y) {
        int sx = x.sign();
        int sy = y.sign();
        if (sx != sy) return sx < sy;
        if (sx < 0) return x.p * y.q > x.q * y.p;
        return x.p * y.q < x.q * y.p;
      }
      fr bool operator<=(p2 x, p2 y) { return !(x > y); }
      fr bool operator>=(p2 x, p2 y) { return !(x < y); }
      fr bool operator==(p2 x, p2 y) { return x.p * y.q == x.q * y.p; }
      fr bool operator>(p2 x, p2 y) {
        int sx = x.sign();
        int sy = y.sign();
        if (sx != sy) return sx > sy;
        if (sx < 0) return x.p * y.q < x.q * y.p;
        return x.p * y.q > x.q * y.p;
      }

      fr double operator*(p2 x, double y) { return (double) x * y; }
      operator double() const { return (double) p / (double) q; }
      int sign() const {
        if (p == T(0)) return 0;
        if ((p < T(0)) == (q < T(0))) return 1;
        return -1;
      }
    };
    // only input int
    template<class T>
    void _R(p2<T> &p) {
      ll x;
      ::inner::IO::_R(x);
      p.p = T(x), p.q = T(1);
    }
    template<class T>
    void _W(const p2<T> &p) { printf("%.11lf", double(p)); }
    // template<class T>
    // p2<T> eps() { return p2<T>(0, 1); }
    template<class T>
    p2<T> nan() { return p2<T>(0, 0); }

    template<class T>
    il int sign(p2<T> k) {
      if (k.p == 0) return 0;
      else if (k < eps<T>())
        return -1;
      return 1;
    }
    // -1: k1 < k2
    // 0: k1 ~ k2
    // 1: k1 > k2
    template<class T>
    il int cmp(p2<T> k1, p2<T> k2) { return sign(k1 - k2); }
    // k3 在 [k1,k2] 内
    template<class T>
    il int inmid(p2<T> k1, p2<T> k2, p2<T> k3) { return sign(k1 - k3) * sign(k2 - k3) <= 0; }
  }// namespace frac_geo
}// namespace inner

#endif
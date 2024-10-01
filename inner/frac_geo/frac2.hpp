#ifndef GEO_DB
#define GEO_DB

#include "inner/prelude"// IWYU pragma: keep
namespace inner {
  namespace frac_geo {

#include <stdckdint.h>
    template<class T = ll>
    struct overflow_t {
      T t;
      fr overflow_t operator+(overflow_t x, overflow_t y) {
        T z;
        auto ov = ckd_add(&z, x.t, y.t);
        assert(!ov);
        return overflow_t(z);
      }
      fr overflow_t operator-(overflow_t x, overflow_t y) {
        T z;
        auto ov = ckd_sub(&z, x.t, y.t);
        assert(!ov);
        return overflow_t(z);
      }
      fr overflow_t operator-(overflow_t x) {
        T z;
        auto ov = ckd_sub(&z, 0, x.t);
        assert(!ov);
        return overflow_t(z);
      }
      fr overflow_t operator*(overflow_t x, overflow_t y) {
        T z;
        auto ov = ckd_mul(&z, x.t, y.t);
        assert(!ov);
        return overflow_t(z);
      }
      operator double() const { return (double) t; }
    };

    template<class T = ll>
    struct p2_overflow {
      overflow_t<T> p, q;// p / q
      p2_overflow(T _p = 0, T _q = 1, bool auto_r = false) {
        if (auto_r) reduction(_p, _q);
        p = overflow_t(_p), q = overflow_t(_q);
      }
      void reduction(T &a, T &b) {
        if (a < 0 && b < 0) a = -a, b = -b;
        if (a == 0) {
          b = 1;
          return;
        }
        T tmp = std::gcd(a, b);
        if (tmp == 0) a = b = 0;
        else
          a /= tmp, b /= tmp;
      }
      fr p2_overflow operator+(p2_overflow x, p2_overflow y) {
        assert(x.q != 0);
        assert(y.q != 0);
        return p2_overflow(x.p * y.q + x.q * y.p, x.q * y.q);
      }
      fr p2_overflow operator-(p2_overflow x, p2_overflow y) {
        assert(x.q != 0);
        assert(y.q != 0);
        return p2_overflow(x.p * y.q - x.q * y.p, x.q * y.q);
      }
      fr p2_overflow operator*(p2_overflow x, p2_overflow y) {
        assert(x.q != 0);
        assert(y.q != 0);
        return p2_overflow(x.p * y.p, x.q * y.q);
      }
      fr p2_overflow operator/(p2_overflow x, p2_overflow y) {
        assert(x.q != 0);
        assert(y.q != 0);
        assert(y.p != 0);
        return p2_overflow(x.p * y.q, x.q * y.p);
      }
      fr p2_overflow operator-(p2_overflow x) {
        assert(x.q != 0);
        return p2_overflow(-x.p, x.q);
      }
      fr bool operator<(p2_overflow x, p2_overflow y) {
        assert(x.q != 0);
        assert(y.q != 0);
        int sx = x.sign();
        int sy = y.sign();
        if (sx != sy) return sx < sy;
        if (sx < 0) return x.p * y.q > x.q * y.p;
        return x.p * y.q < x.q * y.p;
      }
      fr bool operator<=(p2_overflow x, p2_overflow y) {
        assert(x.q != 0);
        assert(y.q != 0);
        return !(x > y);
      }
      fr bool operator>=(p2_overflow x, p2_overflow y) {
        assert(x.q != 0);
        assert(y.q != 0);
        return !(x < y);
      }
      fr bool operator==(p2_overflow x, p2_overflow y) {
        assert(x.q != 0);
        assert(y.q != 0);
        return x.p * y.q == x.q * y.p;
      }
      fr bool operator>(p2_overflow x, p2_overflow y) {
        assert(x.q != 0);
        assert(y.q != 0);
        int sx = x.sign();
        int sy = y.sign();
        if (sx != sy) return sx > sy;
        if (sx < 0) return x.p * y.q < x.q * y.p;
        return x.p * y.q > x.q * y.p;
      }

      fr double operator*(p2_overflow x, double y) {
        assert(x.q != 0);
        return (double) x * y;
      }
      operator float() const { return (float) p / q; }
      operator double() const { return (double) p / q; }
      int sign() const {
        if (p == 0) return 0;
        if ((p < 0) == (q < 0)) return 1;
        return -1;
      }
    };

    template<class T>
    void _W(const p2_overflow<T> &p) { printf("%.11lf", double(p)); }
    // template<class T>
    // p2_overflow<T> eps() { return p2_overflow<T>(0, 1); }
    template<class T>
    p2_overflow<T> nan() { return p2_overflow<T>(0, 0); }

    template<class T>
    il int sign(p2_overflow<T> k) {
      if (k.p == 0) return 0;
      else if (k < eps<T>())
        return -1;
      return 1;
    }
    // -1: k1 < k2
    // 0: k1 ~ k2
    // 1: k1 > k2
    template<class T>
    il int cmp(p2_overflow<T> k1, p2_overflow<T> k2) { return sign(k1 - k2); }
    // k3 在 [k1,k2] 内
    template<class T>
    il int inmid(p2_overflow<T> k1, p2_overflow<T> k2, p2_overflow<T> k3) { return sign(k1 - k3) * sign(k2 - k3) <= 0; }
  }// namespace frac_geo
}// namespace inner

#endif
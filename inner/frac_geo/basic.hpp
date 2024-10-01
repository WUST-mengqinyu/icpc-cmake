#ifndef GEO_BASIC
#define GEO_BASIC

#include "inner/frac_geo/frac.hpp"
#include "inner/num/big_int.hpp"
#include "inner/prelude"// IWYU pragma: keep

namespace inner {
  namespace frac_geo {
#define mp make_pair
#define fi first
#define se second
#define pb push_back
    typedef frac_geo::p2<big_int::bigint> db;
    // typedef frac_geo::p2_overflow<__int128> db;
    const db eps = db(0);
    //const db pi = acos(-1);
    il int sign(db k) {
      if (k > eps) return 1;
      else if (k < -eps)
        return -1;
      return 0;
    }
    // -1: k1 < k2
    // 0: k1 ~ k2
    // 1: k1 > k2
    il int cmp(db k1, db k2) { return sign(k1 - k2); }
    // k3 在 [k1,k2] 内
    il int inmid(db k1, db k2, db k3) { return sign(k1 - k3) * sign(k2 - k3) <= 0; }
    struct point {
      db x, y;
      point operator+(const point &k1) const { return (point){k1.x + x, k1.y + y}; }
      point operator-(const point &k1) const { return (point){x - k1.x, y - k1.y}; }
      point operator*(db k1) const { return (point){x * k1, y * k1}; }
      point operator/(db k1) const { return (point){x / k1, y / k1}; }
      int operator==(const point &k1) const { return cmp(x, k1.x) == 0 && cmp(y, k1.y) == 0; }
      // 逆时针旋转
      //point turn(db k1) { return (point){x * cos(k1) - y * sin(k1), x * sin(k1) + y * cos(k1)}; }
      point turn90() { return (point){-y, x}; }
      bool operator<(const point &k1) const {
        int a = cmp(x, k1.x);
        if (a == -1) return 1;
        else if (a == 1)
          return 0;
        else
          return cmp(y, k1.y) == -1;
      }
      //db abs() { return sqrt(x * x + y * y); }
      db abs2() { return x * x + y * y; }
      //db dis(point k1) { return ((*this) - k1).abs(); }
      // point unit() {
      //   db w = abs();
      //   return (point){x / w, y / w};
      // }

      // db getw() { return atan2(y, x); }
      point getdel() {
        if (sign(x) == -1 || (sign(x) == 0 && sign(y) == -1)) return (*this) * db(-1);
        else
          return (*this);
      }
      int getP() const { return sign(y) == 1 || (sign(y) == 0 && sign(x) == -1); }
    };
    il void _R(point &x) { _R(x.x), _R(x.y); }
    il void _W(const point &x) { _W(x.x), putchar(' '), _W(x.y); }
    il int inmid(point k1, point k2, point k3) { return inmid(k1.x, k2.x, k3.x) && inmid(k1.y, k2.y, k3.y); }
    il db cross(point k1, point k2) { return k1.x * k2.y - k1.y * k2.x; }
    il db dot(point k1, point k2) { return k1.x * k2.x + k1.y * k2.y; }
    //il db rad(point k1, point k2) { return atan2(cross(k1, k2), dot(k1, k2)); }
    // -pi -> pi
    il int compareangle(point k1, point k2) {
      return k1.getP() < k2.getP() || (k1.getP() == k2.getP() && sign(cross(k1, k2)) > 0);
    }
    il point proj(point k1, point k2, point q) {// q 到直线 k1,k2 的投影
      point k = k2 - k1;
      return k1 + k * (dot(q - k1, k) / k.abs2());
    }
    il point reflect(point k1, point k2, point q) { return proj(k1, k2, q) * db(2) - q; }
    il int clockwise(point k1, point k2, point k3) {// k1 k2 k3 逆时针 1 顺时针 -1 否则 0
      return sign(cross(k2 - k1, k3 - k1));
    }
    il int checkLL(point k1, point k2, point k3, point k4) {// 求直线 (L) 线段 (S)k1,k2 和 k3,k4 的交点
      return cmp(cross(k3 - k1, k4 - k1), cross(k3 - k2, k4 - k2)) != 0;
    }
    il point getLL(point k1, point k2, point k3, point k4) {
      db w1 = cross(k1 - k3, k4 - k3), w2 = cross(k4 - k3, k2 - k3);
      return k1 / w1 + k2 / w2;
    }
    il int intersect(db l1, db r1, db l2, db r2) {
      if (l1 > r1) std::swap(l1, r1);
      if (l2 > r2) std::swap(l2, r2);
      return cmp(r1, l2) != -1 && cmp(r2, l1) != -1;
    }
    il int checkSS(point k1, point k2, point k3, point k4) {
      return intersect(k1.x, k2.x, k3.x, k4.x) && intersect(k1.y, k2.y, k3.y, k4.y) &&
             sign(cross(k3 - k1, k4 - k1)) * sign(cross(k3 - k2, k4 - k2)) <= 0 &&
             sign(cross(k1 - k3, k2 - k3)) * sign(cross(k1 - k4, k2 - k4)) <= 0;
    }
    // il db disSP(point k1, point k2, point q) {
    //   point k3 = proj(k1, k2, q);
    //   if (inmid(k1, k2, k3)) return q.dis(k3);
    //   else
    //     return std::min(q.dis(k1), q.dis(k2));
    // }
    // il db disSS(point k1, point k2, point k3, point k4) {
    //   if (checkSS(k1, k2, k3, k4)) return 0;
    //   else
    //     return std::min(std::min(disSP(k1, k2, k3), disSP(k1, k2, k4)), std::min(disSP(k3, k4, k1), disSP(k3, k4, k2)));
    // }
    il int onS(point k1, point k2, point q) { return inmid(k1, k2, q) && sign(cross(k1 - q, k2 - k1)) == 0; }
    struct line {
      // p[0]->p[1]
      point p[2];
      line(point k1, point k2) {
        p[0] = k1;
        p[1] = k2;
      }
      point &operator[](int k) { return p[k]; }
      int include(point k) {
        // cross k1.x * k2.y - k1.y * k2.x
        auto lf = p[1] - p[0];
        auto rf = k - p[0];
        return lf.x * rf.y > lf.y * rf.x;
        // return sign(cross(p[1] - p[0], k - p[0])) > 0;
      }
      point dir() { return p[1] - p[0]; }
      // line push() {// 向外 ( 左手边 ) 平移 eps
      // point delta = (p[1] - p[0]).turn90().unit() * eps;
      // return {p[0] - delta, p[1] - delta};
      // }
    };
    il point getLL(line k1, line k2) { return getLL(k1[0], k1[1], k2[0], k2[1]); }
    il int parallel(line k1, line k2) { return sign(cross(k1.dir(), k2.dir())) == 0; }
    il int sameDir(line k1, line k2) { return parallel(k1, k2) && sign(dot(k1.dir(), k2.dir())) == 1; }
    il int operator<(line k1, line k2) {
      if (sameDir(k1, k2)) return k2.include(k1[0]);
      return compareangle(k1.dir(), k2.dir());
    }
    il int checkpos(line k1, line k2, line k3) { return k3.include(getLL(k1, k2)); }
  }// namespace frac_geo
}// namespace inner


#endif
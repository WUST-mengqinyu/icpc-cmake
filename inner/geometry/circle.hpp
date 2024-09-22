#include "inner/geometry/basic.hpp"
#include "inner/prelude"// IWYU pragma: keep
#include <random>

namespace inner {
  namespace geo {
    struct circle {
      point o;
      db r;
      void scan() {
        o.scan();
        scanf("%lf", &r);
      }
      int inside(point k) { return cmp(r, o.dis(k)); }
    };
    // 两个圆的公切线数量
    il int checkposCC(circle k1, circle k2) {
      if (cmp(k1.r, k2.r) == -1) std::swap(k1, k2);
      db dis = k1.o.dis(k2.o);
      int w1 = cmp(dis, k1.r + k2.r), w2 = cmp(dis, k1.r - k2.r);
      if (w1 > 0) return 4;
      else if (w1 == 0)
        return 3;
      else if (w2 > 0)
        return 2;
      else if (w2 == 0)
        return 1;
      else
        return 0;
    }
    // 沿着 k2->k3 方向给出 , 相切给出两个
    il std::vector<point> getCL(circle k1, point k2, point k3) {
      point k = proj(k2, k3, k1.o);
      db d = k1.r * k1.r - (k - k1.o).abs2();
      if (sign(d) == -1) return {};
      point del = (k3 - k2).unit() * sqrt(std::max((db) 0.0, d));
      return {k - del, k + del};
    }
    // 沿圆 k1 逆时针给出 , 相切给出两个
    il std::vector<point> getCC(circle k1, circle k2) {
      int pd = checkposCC(k1, k2);
      if (pd == 0 || pd == 4) return {};
      db a = (k2.o - k1.o).abs2(), cosA = (k1.r * k1.r + a - k2.r * k2.r) / (2 * k1.r * sqrt(std::max(a, (db) 0.0)));
      db b = k1.r * cosA, c = sqrt(std::max((db) 0.0, k1.r * k1.r - b * b));
      point k = (k2.o - k1.o).unit(), m = k1.o + k * b, del = k.turn90() * c;
      return {m - del, m + del};
    }
    // 沿圆 k1 逆时针给出
    il std::vector<point> TangentCP(circle k1, point k2) {
      db a = (k2 - k1.o).abs(), b = k1.r * k1.r / a, c = sqrt(std::max((db) 0.0, k1.r * k1.r - b * b));
      point k = (k2 - k1.o).unit(), m = k1.o + k * b, del = k.turn90() * c;
      return {m - del, m + del};
    }
    il std::vector<line> TangentoutCC(circle k1, circle k2) {
      int pd = checkposCC(k1, k2);
      if (pd == 0) return {};
      if (pd == 1) {
        point k = getCC(k1, k2)[0];
        return {(line){k, k}};
      }
      if (cmp(k1.r, k2.r) == 0) {
        point del = (k2.o - k1.o).unit().turn90().getdel();
        return {(line){k1.o - del * k1.r, k2.o - del * k2.r}, (line){k1.o + del * k1.r, k2.o + del * k2.r}};
      } else {
        point p = (k2.o * k1.r - k1.o * k2.r) / (k1.r - k2.r);
        std::vector<point> A = TangentCP(k1, p), B = TangentCP(k2, p);
        std::vector<line> ans;
        for (int i = 0; i < A.size(); i++) ans.push_back((line){A[i], B[i]});
        return ans;
      }
    }
    il std::vector<line> TangentinCC(circle k1, circle k2) {
      int pd = checkposCC(k1, k2);
      if (pd <= 2) return {};
      if (pd == 3) {
        point k = getCC(k1, k2)[0];
        return {(line){k, k}};
      }
      point p = (k2.o * k1.r + k1.o * k2.r) / (k1.r + k2.r);
      std::vector<point> A = TangentCP(k1, p), B = TangentCP(k2, p);
      std::vector<line> ans;
      for (int i = 0; i < A.size(); i++) ans.push_back((line){A[i], B[i]});
      return ans;
    }
    il std::vector<line> TangentCC(circle k1, circle k2) {
      int flag = 0;
      if (k1.r < k2.r) std::swap(k1, k2), flag = 1;
      std::vector<line> A = TangentoutCC(k1, k2), B = TangentinCC(k1, k2);
      for (line k: B) A.push_back(k);
      if (flag)
        for (line &k: A) std::swap(k[0], k[1]);
      return A;
    }
    // 圆 k1 与三角形 k2 k3 k1.o 的有向面积交
    il db getarea(circle k1, point k2, point k3) {
      point k = k1.o;
      k1.o = k1.o - k;
      k2 = k2 - k;
      k3 = k3 - k;
      int pd1 = k1.inside(k2), pd2 = k1.inside(k3);
      std::vector<point> A = getCL(k1, k2, k3);
      if (pd1 >= 0) {
        if (pd2 >= 0) return cross(k2, k3) / 2;
        return k1.r * k1.r * rad(A[1], k3) / 2 + cross(k2, A[1]) / 2;
      } else if (pd2 >= 0) {
        return k1.r * k1.r * rad(k2, A[0]) / 2 + cross(A[0], k3) / 2;
      } else {
        int pd = cmp(k1.r, disSP(k2, k3, k1.o));
        if (pd <= 0) return k1.r * k1.r * rad(k2, k3) / 2;
        return cross(A[0], A[1]) / 2 + k1.r * k1.r * (rad(k2, A[0]) + rad(A[1], k3)) / 2;
      }
    }
    il circle getcircle(point k1, point k2, point k3) {
      db a1 = k2.x - k1.x, b1 = k2.y - k1.y, c1 = (a1 * a1 + b1 * b1) / 2;
      db a2 = k3.x - k1.x, b2 = k3.y - k1.y, c2 = (a2 * a2 + b2 * b2) / 2;
      db d = a1 * b2 - a2 * b1;
      point o = (point){k1.x + (c1 * b2 - c2 * b1) / d, k1.y + (a1 * c2 - a2 * c1) / d};
      return (circle){o, k1.dis(o)};
    }
    il circle getScircle(std::vector<point> A) {
      std::mt19937 gen(0);
      std::shuffle(A.begin(), A.end(), gen);
      circle ans = (circle){A[0], 0};
      for (int i = 1; i < A.size(); i++)
        if (ans.inside(A[i]) == -1) {
          ans = (circle){A[i], 0};
          for (int j = 0; j < i; j++)
            if (ans.inside(A[j]) == -1) {
              ans.o = (A[i] + A[j]) / 2;
              ans.r = ans.o.dis(A[i]);
              for (int k = 0; k < j; k++)
                if (ans.inside(A[k]) == -1)
                  ans = getcircle(A[i], A[j], A[k]);
            }
        }
      return ans;
    }
  }// namespace geo
}// namespace inner
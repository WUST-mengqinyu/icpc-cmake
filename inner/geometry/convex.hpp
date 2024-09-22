#include "inner/geometry/basic.hpp"
#include "inner/prelude"// IWYU pragma: keep

namespace inner {
  namespace geo {
    il db area(std::vector<point> A) {// 多边形用 vector<point> 表示 , 逆时针
      db ans = 0;
      for (int i = 0; i < A.size(); i++) ans += cross(A[i], A[(i + 1) % A.size()]);
      return ans / 2;
    }
    il int checkconvex(std::vector<point> A) {
      int n = A.size();
      A.push_back(A[0]);
      A.push_back(A[1]);
      for (int i = 0; i < n; i++)
        if (sign(cross(A[i + 1] - A[i], A[i + 2] - A[i])) == -1) return 0;
      return 1;
    }
    il int contain(std::vector<point> A, point q) {// 2 内部 1 边界 0 外部
      int pd = 0;
      A.push_back(A[0]);
      for (int i = 1; i < A.size(); i++) {
        point u = A[i - 1], v = A[i];
        if (onS(u, v, q)) return 1;
        if (cmp(u.y, v.y) > 0) std::swap(u, v);
        if (cmp(u.y, q.y) >= 0 || cmp(v.y, q.y) < 0) continue;
        if (sign(cross(u - v, q - v)) < 0) pd ^= 1;
      }
      return pd << 1;
    }
    il std::vector<point> ConvexHull(std::vector<point> A, int flag = 1) {// flag=0 不严格 flag=1 严格
      int n = A.size();
      std::vector<point> ans(n * 2);
      sort(A.begin(), A.end());
      int now = -1;
      for (int i = 0; i < A.size(); i++) {
        while (now > 0 && sign(cross(ans[now] - ans[now - 1], A[i] - ans[now - 1])) < flag) now--;
        ans[++now] = A[i];
      }
      int pre = now;
      for (int i = n - 2; i >= 0; i--) {
        while (now > pre && sign(cross(ans[now] - ans[now - 1], A[i] - ans[now - 1])) < flag) now--;
        ans[++now] = A[i];
      }
      ans.resize(now);
      return ans;
    }
    il db convexDiameter(std::vector<point> A) {
      int now = 0, n = A.size();
      db ans = 0;
      for (int i = 0; i < A.size(); i++) {
        now = std::max(now, i);
        while (1) {
          db k1 = A[i].dis(A[now % n]), k2 = A[i].dis(A[(now + 1) % n]);
          ans = std::max(ans, std::max(k1, k2));
          if (k2 > k1) now++;
          else
            break;
        }
      }
      return ans;
    }
    il std::vector<point> convexcut(std::vector<point> A, point k1, point k2) {
      // 保留 k1,k2,p 逆时针的所有点
      int n = A.size();
      A.push_back(A[0]);
      std::vector<point> ans;
      for (int i = 0; i < n; i++) {
        int w1 = clockwise(k1, k2, A[i]), w2 = clockwise(k1, k2, A[i + 1]);
        if (w1 >= 0) ans.push_back(A[i]);
        if (w1 * w2 < 0) ans.push_back(getLL(k1, k2, A[i], A[i + 1]));
      }
      return ans;
    }
    il int checkPoS(std::vector<point> A, point k1, point k2) {
      // 多边形 A 和直线 ( 线段 )k1->k2 严格相交 , 注释部分为线段
      struct ins {
        point m, u, v;
        int operator<(const ins &k) const { return m < k.m; }
      };
      std::vector<ins> B;
      //if (contain(A,k1)==2||contain(A,k2)==2) return 1;
      std::vector<point> poly = A;
      A.push_back(A[0]);
      for (int i = 1; i < A.size(); i++)
        if (checkLL(A[i - 1], A[i], k1, k2)) {
          point m = getLL(A[i - 1], A[i], k1, k2);
          if (inmid(A[i - 1], A[i], m) /*&&inmid(k1,k2,m)*/) B.push_back((ins){m, A[i - 1], A[i]});
        }
      if (B.size() == 0) return 0;
      sort(B.begin(), B.end());
      int now = 1;
      while (now < B.size() && B[now].m == B[0].m) now++;
      if (now == B.size()) return 0;
      int flag = contain(poly, (B[0].m + B[now].m) / 2);
      if (flag == 2) return 1;
      point d = B[now].m - B[0].m;
      for (int i = now; i < B.size(); i++) {
        if (!(B[i].m == B[i - 1].m) && flag == 2) return 1;
        int tag = sign(cross(B[i].v - B[i].u, B[i].m + d - B[i].u));
        if (B[i].m == B[i].u || B[i].m == B[i].v) flag += tag;
        else
          flag += tag * 2;
      }
      //return 0;
      return flag == 2;
    }
    il int checkinp(point r, point l, point m) {
      if (compareangle(l, r)) { return compareangle(l, m) && compareangle(m, r); }
      return compareangle(l, m) || compareangle(m, r);
    }
    // 快速检查线段是否和多边形严格相交
    il int checkPosFast(std::vector<point> A, point k1, point k2) {
      if (contain(A, k1) == 2 || contain(A, k2) == 2) return 1;
      if (k1 == k2) return 0;
      A.push_back(A[0]);
      A.push_back(A[1]);
      for (int i = 1; i + 1 < A.size(); i++)
        if (checkLL(A[i - 1], A[i], k1, k2)) {
          point now = getLL(A[i - 1], A[i], k1, k2);
          if (inmid(A[i - 1], A[i], now) == 0 || inmid(k1, k2, now) == 0) continue;
          if (now == A[i]) {
            if (A[i] == k2) continue;
            point pre = A[i - 1], ne = A[i + 1];
            if (checkinp(pre - now, ne - now, k2 - now)) return 1;
          } else if (now == k1) {
            if (k1 == A[i - 1] || k1 == A[i]) continue;
            if (checkinp(A[i - 1] - k1, A[i] - k1, k2 - k1)) return 1;
          } else if (now == k2 || now == A[i - 1])
            continue;
          else
            return 1;
        }
      return 0;
    }
  }// namespace geo
}// namespace inner
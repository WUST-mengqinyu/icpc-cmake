#include "inner/geometry/basic.hpp"
#include "inner/prelude"// IWYU pragma: keep

namespace inner {
  namespace geo {
    // 拆分凸包成上下凸壳 凸包尽量都随机旋转一个角度来避免出现相同横坐标
    // 尽量特判只有一个点的情况 凸包逆时针
    il void getUDP(std::vector<point> A, std::vector<point> &U, std::vector<point> &D) {
      db l = 1e100, r = -1e100;
      for (int i = 0; i < A.size(); i++) l = std::min(l, A[i].x), r = std::max(r, A[i].x);
      int wherel, wherer;
      for (int i = 0; i < A.size(); i++)
        if (cmp(A[i].x, l) == 0) wherel = i;
      for (int i = A.size(); i; i--)
        if (cmp(A[i - 1].x, r) == 0) wherer = i - 1;
      U.clear();
      D.clear();
      int now = wherel;
      while (1) {
        D.push_back(A[now]);
        if (now == wherer) break;
        now++;
        if (now >= A.size()) now = 0;
      }
      now = wherel;
      while (1) {
        U.push_back(A[now]);
        if (now == wherer) break;
        now--;
        if (now < 0) now = A.size() - 1;
      }
    }
    // 需要保证凸包点数大于等于 3,2 内部 ,1 边界 ,0 外部
    il int containCoP(const std::vector<point> &U, const std::vector<point> &D, point k) {
      db lx = U[0].x, rx = U[U.size() - 1].x;
      if (k == U[0] || k == U[U.size() - 1]) return 1;
      if (cmp(k.x, lx) == -1 || cmp(k.x, rx) == 1) return 0;
      int where1 = lower_bound(U.begin(), U.end(), (point){k.x, -1e100}) - U.begin();
      int where2 = lower_bound(D.begin(), D.end(), (point){k.x, -1e100}) - D.begin();
      int w1 = clockwise(U[where1 - 1], U[where1], k), w2 = clockwise(D[where2 - 1], D[where2], k);
      if (w1 == 1 || w2 == -1) return 0;
      else if (w1 == 0 || w2 == 0)
        return 1;
      return 2;
    }
    // d 是方向 , 输出上方切点和下方切点
    il std::pair<point, point> getTangentCow(const std::vector<point> &U, const std::vector<point> &D, point d) {
      if (sign(d.x) < 0 || (sign(d.x) == 0 && sign(d.y) < 0)) d = d * (-1);
      point whereU, whereD;
      if (sign(d.x) == 0) return std::mp(U[0], U[U.size() - 1]);
      int l = 0, r = U.size() - 1, ans = 0;
      while (l < r) {
        int mid = l + r >> 1;
        if (sign(cross(U[mid + 1] - U[mid], d)) <= 0) l = mid + 1, ans = mid + 1;
        else
          r = mid;
      }
      whereU = U[ans];
      l = 0, r = D.size() - 1, ans = 0;
      while (l < r) {
        int mid = l + r >> 1;
        if (sign(cross(D[mid + 1] - D[mid], d)) >= 0) l = mid + 1, ans = mid + 1;
        else
          r = mid;
      }
      whereD = D[ans];
      return std::mp(whereU, whereD);
    }
    // 先检查 contain, 逆时针给出
    il std::pair<point, point> getTangentCoP(const std::vector<point> &U, const std::vector<point> &D, point k) {
      db lx = U[0].x, rx = U[U.size() - 1].x;
      if (k.x < lx) {
        int l = 0, r = U.size() - 1, ans = U.size() - 1;
        while (l < r) {
          int mid = l + r >> 1;
          if (clockwise(k, U[mid], U[mid + 1]) == 1) l = mid + 1;
          else
            ans = mid, r = mid;
        }
        point w1 = U[ans];
        l = 0, r = D.size() - 1, ans = D.size() - 1;
        while (l < r) {
          int mid = l + r >> 1;
          if (clockwise(k, D[mid], D[mid + 1]) == -1) l = mid + 1;
          else
            ans = mid, r = mid;
        }
        point w2 = D[ans];
        return std::mp(w1, w2);
      } else if (k.x > rx) {
        int l = 1, r = U.size(), ans = 0;
        while (l < r) {
          int mid = l + r >> 1;
          if (clockwise(k, U[mid], U[mid - 1]) == -1) r = mid;
          else
            ans = mid, l = mid + 1;
        }
        point w1 = U[ans];
        l = 1, r = D.size(), ans = 0;
        while (l < r) {
          int mid = l + r >> 1;
          if (clockwise(k, D[mid], D[mid - 1]) == 1) r = mid;
          else
            ans = mid, l = mid + 1;
        }
        point w2 = D[ans];
        return std::mp(w2, w1);
      } else {
        int where1 = lower_bound(U.begin(), U.end(), (point){k.x, -1e100}) - U.begin();
        int where2 = lower_bound(D.begin(), D.end(), (point){k.x, -1e100}) - D.begin();
        if ((k.x == lx && k.y > U[0].y) || (where1 && clockwise(U[where1 - 1], U[where1], k) == 1)) {
          int l = 1, r = where1 + 1, ans = 0;
          while (l < r) {
            int mid = l + r >> 1;
            if (clockwise(k, U[mid], U[mid - 1]) == 1) ans = mid, l = mid + 1;
            else
              r = mid;
          }
          point w1 = U[ans];
          l = where1, r = U.size() - 1, ans = U.size() - 1;
          while (l < r) {
            int mid = l + r >> 1;
            if (clockwise(k, U[mid], U[mid + 1]) == 1) l = mid + 1;
            else
              ans = mid, r = mid;
          }
          point w2 = U[ans];
          return std::mp(w2, w1);
        } else {
          int l = 1, r = where2 + 1, ans = 0;
          while (l < r) {
            int mid = l + r >> 1;
            if (clockwise(k, D[mid], D[mid - 1]) == -1) ans = mid, l = mid + 1;
            else
              r = mid;
          }
          point w1 = D[ans];
          l = where2, r = D.size() - 1, ans = D.size() - 1;
          while (l < r) {
            int mid = l + r >> 1;
            if (clockwise(k, D[mid], D[mid + 1]) == -1) l = mid + 1;
            else
              ans = mid, r = mid;
          }
          point w2 = D[ans];
          return std::mp(w1, w2);
        }
      }
    }
  }// namespace geo
}// namespace inner
#include "inner/frac_geo/basic.hpp"
#include "inner/prelude"// IWYU pragma: keep

namespace inner {
  namespace frac_geo {
    // 求半平面交，逆时针返回原始 line index
    il std::vector<int> half_plane_only_ord(const std::vector<line> &l) {
      std::vector<int> idx(l.size());
      std::iota(ALL(idx), 0);
      sort(ALL(idx), [&](int x, int y) { return l[x] < l[y]; });
      std::vector<int> q(l.size() + 1);
      int L = 0, R = -1;
      for (int i = 0; i < (int) l.size(); ++i) {
        if (i && sameDir(l[idx[i]], l[idx[i - 1]])) continue;
        while (L < R && !checkpos(l[q[R - 1]], l[q[R]], l[idx[i]])) R--;
        while (L < R && !checkpos(l[q[L + 1]], l[q[L]], l[idx[i]])) L++;
        q[++R] = idx[i];
      }
      while (L + 1 < R && !checkpos(l[q[R - 1]], l[q[R]], l[q[L]])) R--;
      while (L + 1 < R && !checkpos(l[q[L + 1]], l[q[L]], l[q[R]])) L++;
      return std::vector(q.begin() + L, q.begin() + R + 1);
    }

    // 求半平面交, 返回交点， 半平面是逆时针方向 , 输出按照逆时针，点数小于等于2表示无法构成半平面交
    il std::vector<point> half_plane(const std::vector<line> &l) {
      auto q = half_plane_only_ord(l);
      std::vector<point> ret;
      for (int i = 0; i < (int) q.size(); ++i) ret.push_back(getLL(l[q[i]], l[q[(i + 1) % q.size()]]));
      return ret;
    }
  }// namespace frac_geo
}// namespace inner
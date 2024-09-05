#include "atcoder/fenwicktree.hpp"
#include "inner/prelude"

using namespace std;

namespace ODT {
  template<class T>
  struct node {
    int l, r;
    mutable T v;
    node(int L, int R = -1, T V = 0) : l(L), r(R), v(V) {}
    bool operator<(const node &o) const {
      return l < o.l;
    }
  };
  template<class T>
  struct odt {
    typedef set<node<T>>::iterator IT;
    set<node<T>> s;
    IT split(int pos, int v) {
      IT it = prev(s.upper_bound(node<T>(pos)));
      int L = it->l, R = it->r;
      T V = it->v;
      s.erase(it);
      if (L <= pos - 1) s.insert(node(L, pos - 1, V));
      if (R >= pos + 1) s.insert(node(pos + 1, R, V));
      return s.insert(node(pos, pos, v)).first;
    }
    void set(int p, T v) {
      IT it = split(p, v);
      it->v = v;
      if (it != s.begin() && prev(it)->v == v) {
        int l = prev(it)->l;
        s.erase(prev(it), next(it));
        auto [itn, _b] = s.insert(node(l, p, v));
        it = itn;
        p = l;
      }
      if (next(it) != s.end() && next(it)->v == v) {
        int r = next(it)->r;
        s.erase(it, next(next(it)));
        s.insert(node(p, r, v));
      }
    }
    IT find(int pos) {
      return prev(s.upper_bound(node<T>(pos)));
    }
  };

}// namespace ODT

MAIN() {
  int n;
  R(n);
  vector<int> a(n), b(n);
  R(a, b);
  atcoder::fenwick_tree<ll> sx(n + 1);
  ODT::odt<int> sy;
  int yp = 0;
  for (int i = 0; i < n; ++i) {
    sx.add(i + 1, a[i]);
    if (b[yp] == b[i]) {
      continue;
    } else {
      sy.s.insert(ODT::node(yp + 1, i, b[yp]));
      yp = i;
    }
  }
  sy.s.insert(ODT::node(yp + 1, n, b[yp]));
  int q;
  R(q);
  while (q--) {
    int op, x, y;
    R(op, x, y);
    if (op == 1) {
      sx.add(x, y - a[x - 1]);
      a[x - 1] = y;
    } else if (op == 2) {
      sy.set(x, y);
    } else {
      auto it = sy.find(x);
      int nowp = x;
      ll res = 0;
      while (it != sy.s.end() && nowp <= y) {
        int R = min(it->r, y);
        if (it->v == 1) {
          res += sx.sum(nowp, R + 1);
          nowp = R + 1;
        } else {
          while (nowp <= R) {
            res = max(res + a[nowp - 1], res * it->v);
            nowp++;
          }
        }
        it = next(it);
      }
      W(res);
    }
  }
}
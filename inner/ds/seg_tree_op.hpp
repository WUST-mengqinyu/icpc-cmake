#include "inner/prelude"

namespace inner {
  namespace seg_tree_op {
#include "inner/num/bit.hpp"

    struct NullTag {
      NullTag() {};
      const bool operator==(const NullTag &oth) {
        return true;
      }
      template<class T>
      friend T operator+=(const T &a, const NullTag &b) {
        return a;
      }
      template<class T>
      friend T operator+=(const NullTag &a, const T &b) {
        return b;
      }

      NullTag operator+=(const NullTag &oth) {
        return NullTag{};
      }
    };

    template<class S, class T>
    concept SegTreeNode = requires(S s, T t) {
      std::default_initializable<S>;
      std::copy_constructible<S>;
      std::default_initializable<T>;
      s += t;
      t += t;
    };


    template<const int N, class Num>
    struct PolyTag {
      std::array<Num, N> tg;
    };


#define OpGen(ClassName, Op)                                                    \
  template<class Num>                                                           \
  struct ClassName {                                                            \
    const int dig = 0;                                                          \
    ClassName(Num x) : x(x) {}                                                  \
    ClassName(Num x, int poly_dig) : x(x), dig(poly_dig) {}                     \
    Num x;                                                                      \
    template<const int N>                                                       \
    friend PolyTag<N, Num> operator+=(PolyTag<N, Num> &x, const ClassName &y) { \
      x.tg[y.dig] Op y.x;                                                       \
      return x;                                                                 \
    }                                                                           \
    friend Num operator+=(Num &x, const ClassName &y) {                         \
      x Op y.x;                                                                 \
      return x;                                                                 \
    }                                                                           \
  }// namespace seg_tree_op

    OpGen(Set, =);
    OpGen(Add, +=);
    OpGen(Mul, *=);
    OpGen(Div, /=);
    OpGen(Mod, %=);

    // need provides: S{}, S{_S_copy}, T{}, T == T(only need is zero), S += S, S += T, T += T, T += Op
    template<class S, class T>
      requires(SegTreeNode<S, T>)
    struct seg {
  public:
      std::vector<S> p;
      int n;

#define pdown_x(x) \
  for (int i = lg; i >= 1; --i) pdown(x >> i);

#define pdown_lr(l, r)                             \
  for (int i = lg; i >= 1; --i) {                  \
    if (((l >> i) << i) != l) pdown(l >> i);       \
    if (((r >> i) << i) != r) pdown((r - 1) >> i); \
  }

#define pup_x(x) \
  for (int i = 1; i <= lg; ++i) pup(x >> i);

      seg() : seg(0) {
      }
      explicit seg(int _n) : seg(std::vector<S>(_n, S{})) {}
      explicit seg(const std::vector<S> &v) : n(int(v.size())) {
        lg = inner::num_bit::ceil_pow2(n);
        sz = 1 << lg;
        p = std::vector<S>(sz << 1, S{});
        in_use = std::vector<bool>(sz << 1, false);
        tg = std::vector<T>(sz, T{});
        for (int i = 0; i < n; ++i) p[sz + i] = v[i], in_use[sz + i] = true;
        for (int i = sz - 1; i >= 1; --i) {
          in_use[i] = in_use[i << 1] | in_use[i << 1 | 1];
          if (in_use[i]) pup(i);
        }
      }

      void set(int x, S nw) {
        assert(1 <= x && x <= n);
        x += sz - 1;
        pdown_x(x);
        p[x] = nw;
        pup_x(x);
      }

      S get(int x) {
        assert(0 <= x && x < n);
        x += sz - 1;
        pdown_x(x);
        return p[x];
      }

      S sum(int l, int r) {
        assert(1 <= l && l <= r && r <= n);
        l += sz - 1;
        r += sz;
        pdown_lr(l, r);
        S ls{}, rs{};

        int found = 0;
        for (; l < r; l >>= 1, r >>= 1) {
          if (l & 1) {
            if (found & 1) ls += p[l++];
            else {
              ls = p[l++];
              found |= 1;
            }
          }
          if (r & 1) {
            if (found & 2) {
              S tmp = p[--r];
              tmp += rs;
              rs = tmp;
            } else {
              rs = p[--r];
              found |= 2;
            }
          }
        }
        S res{};
        if (found & 1) {
          res = ls;
          if (found & 2) res += rs;
        } else if (found & 2)
          res = rs;
        return res;
      }

      S sum() {
        return p[1];
      }

      // binary search, complex $logn$
      std::pair<int, S> bs(std::function<bool(S)> goes, int limit_p = -1, bool from_left = true) {
        // todo;
      }

      template<class Op>
      void apply(int x, Op t) {
        assert(1 <= x && x <= n);
        x += sz - 1;
        pdown_x(x);
        p[x] += t;
        pup_x(x);
      }

      template<class Op>
      void apply(int l, int r, Op t) {
        assert(1 <= l && l <= r && r <= n);
        l += sz - 1;
        r += sz;
        pdown_lr(l, r);
        int l2 = l, r2 = r;
        while (l < r) {
          if (l & 1) _apply_tag(l++, t);
          if (r & 1) _apply_tag(--r, t);
          l >>= 1, r >>= 1;
        }
        l = l2, r = r2;
        for (int i = 1; i <= lg; ++i) {
          if (((l >> i) << i) != l) pup(l >> i);
          if (((r >> i) << i) != r) pup((r - 1) >> i);
        }
      }

      void dbg(std::function<void(S, int l, int r)> const &W, bool with_print = true, bool with_pdown = true,
               bool with_endl = false) {
        if (with_print) std::printf("-----------------\n");
        if (with_pdown)
          for (int i = 1; i < sz; ++i) pdown(i);
        std::vector<std::array<int, 2>> rng(sz + n);
        for (int i = 0; i < n; ++i) rng[sz + i] = {i + 1, i + 1};
        for (int i = sz - 1; i >= 1; --i)
          if (in_use[i]) {
            rng[i][0] = rng[i << 1][0];
            if (in_use[i << 1 | 1])
              rng[i][1] = rng[i << 1 | 1][1];
            else
              rng[i][1] = rng[i << 1][1];
          }
        for (int i = 1; i <= sz + n; ++i) {
          if (!in_use[i]) continue;
          if (with_print) printf("node %d: range[%d-%d]: ", i, rng[i][0], rng[i][1]);
          W(p[i], rng[i][0], rng[i][1]);
          if (with_print && with_endl) std::printf("\n");
        }
        if (with_print) std::printf("-----------------\n");
        fflush(stdout);
      }

  private:
      void _apply_tag(int x, const T &t) {
        p[x] += t;
        if (x < sz) tg[x] += t;
      }
      void pup(int x) {
        if (!in_use[x]) return;
        int l = x << 1;
        if (in_use[l]) {
          p[x] = p[l];
          if (in_use[l | 1]) { p[x] += p[l | 1]; }
        } else if (in_use[l | 1]) {
          p[x] = p[l | 1];
        }
      }
      void pdown(int x) {
        if (tg[x] == T{}) return;
        _apply_tag(x << 1, tg[x]);
        _apply_tag(x << 1 | 1, tg[x]);
        tg[x] = T{};
      }
      std::vector<T> tg;
      std::vector<bool> in_use;
      int sz{}, lg{};
    };// namespace seg_tree_op
  }// namespace seg_tree_op
}// namespace inner
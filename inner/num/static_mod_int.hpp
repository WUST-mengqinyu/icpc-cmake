#include "inner/prelude"

// clang-format off
namespace inner {
  namespace static_mod_int {
#include "inner/num/type_traits.hpp"
    using namespace inner::type_traits;
    template<int m, std::enable_if_t<(1 <= m)> * = nullptr>struct s_mint {
      #define O operator
      #define F friend
      #define C const
    public:
      static constexpr int mod() { return m; }
      static s_mint raw(int v) {s_mint x;x._v = v;return x;}
      s_mint() : _v(0) {}
      template<class T, IS_SI_T<T> * = nullptr>s_mint(T v) {ll x = (ll) (v % (ll) (umod()));if (x < 0) x += umod();_v = (UI) (x);}
      template<class T, IS_UI_T<T> * = nullptr>s_mint(T v) {_v = (UI) (v % umod());}
      UI val() C { return _v; }
      s_mint &O++() {if (++_v == umod()) _v = 0;return *this;}
      s_mint &O--() {if (_v == 0) _v = umod();_v--;return *this;}
      s_mint O++(int) {s_mint v = *this;++*this;return v;}
      s_mint O--(int) {s_mint v = *this;--*this;return v;}
      s_mint &O+=(C s_mint &rhs) {_v += rhs._v;if (_v >= umod()) _v -= umod();return *this;}
      s_mint &O-=(C s_mint &rhs) {_v -= rhs._v;if (_v >= umod()) _v += umod();return *this;}
      s_mint &O*=(C s_mint &rhs) {ULL z = _v;z *= rhs._v;_v = (UI) (z % umod());return *this;}
      s_mint &O/=(C s_mint &rhs) { return *this = *this * rhs.inv(); }
      s_mint O+() C { return *this; }
      s_mint O-() C { return s_mint() - *this; }
      s_mint pow(ll n) C {s_mint x = *this, r = 1;while (n) {if (n & 1) r *= x;x *= x;n >>= 1;}return r;}
      s_mint inv() C {return pow(umod() - 2);}
      F s_mint O+(C s_mint &lhs, C s_mint &rhs) {return s_mint(lhs) += rhs;}
      F s_mint O-(C s_mint &lhs, C s_mint &rhs) {return s_mint(lhs) -= rhs;}
      F s_mint O*(C s_mint &lhs, C s_mint &rhs) {return s_mint(lhs) *= rhs;}
      F s_mint O/(C s_mint &lhs, C s_mint &rhs) {return s_mint(lhs) /= rhs;}
      F bool O==(C s_mint &lhs, C s_mint &rhs) {return lhs._v == rhs._v;}
      F bool O!=(C s_mint &lhs, C s_mint &rhs) {return lhs._v != rhs._v;}
    private:UI _v;static constexpr UI umod() { return m; }
      #undef O
      #undef F
    };
    using modint998244353 = s_mint<998244353>;
    using modint1000000007 = s_mint<1000000007>;
  }// namespace static_mod_int
}// namespace inner
// clang-format on

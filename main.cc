/*================================================================
*
*   创 建 者： badcw
*   创建日期： 2024-07-19 22:38:42
*
================================================================*/
#include <bits/stdc++.h>
//#include "testlib.h"
//#include "atcoder/all"

#define VI vector<int>
#define ll long long
using namespace std;

// clang-format off
namespace IO {
    template<class T>
    void _R(T &x) { cin >> x; }
    void _R(int &x) { scanf("%d", &x); }
    void _R(ll &x) { scanf("%lld", &x); }
    void _R(double &x) { scanf("%lf", &x); }
    void _R(char &x) { x = getchar(); }
    void _R(char *x) { scanf("%s", x); }
    template<class T, class U>
    void _R(pair<T, U> &x) { _R(x.first), _R(x.second); }
    template<class T>
    void _R(vector<T> &x) { for(auto&i:x) _R(i); }
    void R() {}
    template<class T, class... U>
    void R(T &head, U &... tail) {_R(head),R(tail...);}
    template<class T>
    void _W(const T &x) { cout << x; }
    void _W(const int &x) { printf("%d", x); }
    void _W(const ll &x) { printf("%lld", x); }
    void _W(const double &x) { printf("%.16f", x); }
    void _W(const char &x) { putchar(x); }
    void _W(const char *x) { printf("%s", x); }
    template<class T, class U>
    void _W(const pair<T, U> &x) {_W(x.first),putchar(' '),_W(x.second);}
    template<class T>
    void _W(const set<T> &x) { for (auto i = x.begin(); i != x.end(); _W(*i++)) if (i != x.cbegin()) putchar(' '); }
    template<class T>
    void _W(const vector<T> &x) { for (auto i = x.begin(); i != x.end(); _W(*i++)) if (i != x.cbegin()) putchar(' '); }
    void W() {}
    template<class T, class... U>
    void W(const T &head, const U &... tail) {_W(head),putchar(sizeof...(tail) ? ' ' : '\n'),W(tail...);}
}
using namespace IO;
// clang-format on


const int maxn = 1e5+50;
const int mod = 1e9+7;

ll qp(ll a, ll n) {
    ll res = 1;
    n %= mod - 1;
    if (n < 0) n += mod - 1;
    while (n > 0) {
        if (n & 1) res = res * a % mod;
        a = a * a % mod;
        n >>= 1;
    }
    return res;
}

ll qp(ll a, ll n, int mod) {
    ll res = 1;
    n %= mod - 1;
    if (n < 0) n += mod - 1;
    while (n > 0) {
        if (n & 1) res = res * a % mod;
        a = a * a % mod;
        n >>= 1;
    }
    return res;
}

int main(int argc, char **agrv) {
    int a; R(a);
    W(a);
    return 0;
}

/*================================================================
*
*   创 建 者： badcw
*   创建日期： 2022/8/6 下午11:14
*
================================================================*/
#include <bits/stdc++.h>
#ifdef badcw
#include "testlib.h"
#include "atcoder/all"
#endif

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

const int maxn = 1e6+50;
const int mod  = 1e9 + 7;

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

//int tr[maxn][2];
int dp[maxn], pos[maxn];
int p[maxn << 2];
//void dwn(int rt) {
//    if (p[rt]) p[rt << 1] = p[rt << 1 | 1] = 1;
//}
//void upd(int rt, int l, int r, int lx, int rx) {
//    if (lx <= l && r <= rx) {
//        p[rt] = 1;
//        return;
//    }
//    int mid = l + r >> 1;
//    dwn(rt);
//    if (lx <= mid) upd(rt << 1, l, mid, lx, rx);
//    if (rx > mid) upd(rt << 1 | 1, mid + 1, r, lx, rx);
//}

void upd(int x) {
    int &px = pos[x];
    while (px > 0) {
        if (p[px ^ 1]) {
            px >>= 1;
        } else {
            break;
        }
    }
    p[px] = 1;
}

void mark(int px, int ned) {
    while (dp[pos[px]] >= ned) {
//        W("**", tr[pos[px] ^ 1][0], tr[pos[px] ^ 1][1]);
        p[pos[px] ^ 1] = 1;
        pos[px] /= 2;
    }
}

int nxt(int rt, int l, int r, int px) {
    if (r < px) return -1;
    if (p[rt] == 1) return -1;
    if (l == r) return l;
    int mid = l + r >> 1;
    int res = -1;
    if (px <= mid) res = nxt(rt << 1, l, mid, px);
    if (res != -1) return res;
    return nxt(rt << 1 | 1, mid + 1, r, px);
}

int pre(int rt, int l, int r, int px) {
    if (l > px) return -1;
    if (p[rt] == 1) return -1;
    if (l == r) return l;
    int mid = l + r >> 1;
    int res = -1;
    if (px > mid) res = pre(rt << 1 | 1, mid + 1, r, px);
    if (res != -1) return res;
    return pre(rt << 1, l, mid, px);
}

void build(int rt, int l, int r, int dep) {
//    tr[rt][0] = l, tr[rt][1] = r;
    dp[rt] = dep;
    p[rt] = 0;
    if (l == r) {
        pos[l] = rt;
        return;
    }
    int mid = l + r >> 1;
    build(rt << 1, l, mid, dep + 1);
    build(rt << 1 | 1, mid + 1, r, dep + 1);
}

int hasQ = 0;
int n;
#ifdef badcw
VI gg;

int gen(int rt, int l, int r) {
    if (l == r) {
        return l;
    }
    int mid = l + r >> 1;
    int t1 = gen(rt << 1, l, mid);
    int t2 = gen(rt << 1 | 1, mid + 1, r);
    int f = rnd.next(2);
    if (f) {
        gg[t1] ++;
        return t1;
    }
    else {
        gg[t2] ++;
        return t2;
    }
}

int mockQ(int x, int y) {
    hasQ++;
    if (gg.empty()) {
        gg.resize(1+n);
        gen(1, 1, n);
    }
//    assert(x >= 1 && y >= 1 && x <= n && y <= n && x != y);
    if (gg[x] == gg[y]) {
//        W("*", x, y, 0);
        return 0;
    }
    if (gg[x] > gg[y]) {
//        W("*", x, y, 1);
        return 1;
    }
    if (gg[x] < gg[y]) {
//        W("*", x, y, 2);
        return 2;
    }
    return -1;
}

void mockA(int x) {
    int ox = x;
    for (int i = 1; i <= n; ++i) {
        if (gg[i] > gg[x]) {
            x = i;
        }
    }
    W("!", ox, x, hasQ);
    if (ox != x) {
        W(gg);
        exit(-1);
    }
    gg.clear();
    hasQ = 0;
}
#endif

int Q(int x, int y) {
#ifdef badcw
    return mockQ(x, y);
#endif
    printf("? %d %d\n", x, y);
    fflush(stdout);
    int g; R(g);
//    if (g < 0 || g > 2) exit(211);
    return g;
}

void A(int x) {
#ifdef badcw
    mockA(x);
    return;
#endif
    printf("! %d\n", x);
    fflush(stdout);
    assert(hasQ * 2 <= n);
    hasQ = 0;
}

int main(int argc, char** agrv) {
    int T;
#ifdef badcw
    T = 10;
#else
    scanf("%d", &T);
#endif
    for (int kase = 1; kase <= T; ++kase) {
#ifdef badcw
        n = 17;
#else
        R(n);
#endif
        n = 1<<n;
        build(1, 1, n, 0);
        int x = 1, y = n, win = -1;
        while (x != -1 && y != -1) {
            int t = Q(x, y);
            if (t == 0) {
                mark(x, dp[pos[y]]+1);
                mark(y, dp[pos[x]]+1);
                upd(x); upd(y);
                x = nxt(1, 1, n, x), y = pre(1, 1, n, y);
            } else if (t == 1) {
                upd(y);
                mark(x, dp[pos[y]]);
                win = x;
                y = pre(1, 1, n, y);
            } else {
                upd(x);
                mark(y, dp[pos[x]]);
                win = y;
                x = nxt(1, 1, n, x);
            }
            if (x >= y) {
                break;
            }
//            if (x == y && x == n) {
//                y = -1;
//                break;
//            }
//            if (x == y) y = nxt(1, 1, n, y + 1);
        }
//        assert(nxt(1, 1, n, 0) == x + y + 1);
        A(win);
    }
    return 0;
}

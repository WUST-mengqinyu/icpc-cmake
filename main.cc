/*================================================================
*
*   创 建 者： badcw
*   创建日期： 2021/7/27 3:45 下午
*
================================================================*/
#include <bits/stdc++.h>
//#include "testlib.h"
//#include "atcoder/all"

#define VI vector<int>
#define ll long long
using namespace std;

namespace IO {
    template<class T>
    void _R(T &x) { cin >> x; }
    void _R(int &x) { scanf("%d", &x); }
    void _R(ll &x) { scanf("%lld", &x); }
    void _R(double &x) { scanf("%lf", &x); }
    void _R(char &x) { x = getchar(); }
    void _R(char *x) { scanf("%s", x); }
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
    void _W(const vector<T> &x) { for (auto i = x.begin(); i != x.end(); _W(*i++)) if (i != x.cbegin()) putchar(' '); }
    void W() {}
    template<class T, class... U>
    void W(const T &head, const U &... tail) {_W(head),putchar(sizeof...(tail) ? ' ' : '\n'),W(tail...);}
}
using namespace IO;


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

int n, m;
int vis[maxn], fa[maxn];
int F(int x) {
    return x == fa[x] ? x : fa[x] = F(fa[x]);
}

struct edge {
    int u, v, w;
    bool operator < (const edge &oth) const {
        return w < oth.w;
    }
};

int main(int argc, char **agrv) {
    int T; R(T);
    for (int kase = 1; kase <= T; ++kase) {
        R(n, m);
        for (int i = 1; i <= n; ++i) {
            fa[i] = i;
            vis[i] = 0;
        }
        vector<edge> a;
        for (int i = 0; i < m; ++i) {
            int u, v, w;
            R(u, v, w);
            if (u == v) continue;
            a.push_back({u, v, w});
        }
        int k; R(k);
        for (int i = 0; i < k; ++i) {
            int x; R(x);
            vis[x] = 1;
        }
        vector<edge> b;
        ll w = 0;
        int tot = 0;
        sort(a.begin(), a.end());
        if (n == 2) {
            if ((int)a.size() > 0) {
                W(a[0].w);
            } else W(-1);
            continue;
        }
        for (auto &i : a) {
            if (vis[i.u] || vis[i.v]) {
                b.push_back(i);
            } else {
                int u = F(i.u);
                int v = F(i.v);
                if (u != v) {
                    fa[u] = v;
                    w += i.w;
                    tot ++;
//                    W(i.u, i.v, i.w);
                }
            }
        }
        if (tot != n - k - 1) W("Impossible");
        else {
            for (auto &i : b) {
                if (vis[i.u] && vis[i.v]) {
                    continue;
                }
                int u = F(i.u);
                int v = F(i.v);
                if (u != v) {
                    fa[u] = v;
                    w += i.w;
                    tot++;
//                    W(i.u, i.v, i.w);
                }
            }
            if (tot != n - 1) W("Impossible");
            else W(w);
        }
    }
    return 0;
}

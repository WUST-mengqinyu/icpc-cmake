/*================================================================
*
*   创 建 者： badcw
*   创建日期： 2020/10/31 14:06
*
================================================================*/
#include <bits/stdc++.h>

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


const int maxn = 2e5+50;
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
ll p[maxn << 2];
int a[maxn], b[maxn];

void update(int rt, int l, int r, int pos, int k) {
    if (l == r) {
        p[rt] = k;
        return;
    }
    int mid = l + r >> 1;
    if (pos <= mid) update(rt << 1, l, mid, pos, k);
    else update(rt << 1 | 1, mid + 1, r, pos, k);
    p[rt] = p[rt << 1] + p[rt << 1 | 1];
}

ll query(int rt, int l, int r, int le, int re) {
    if (re == 0) return 0;
    if (le <= l && r <= re) return p[rt];
    ll sum = 0;
    int mid = l + r >> 1;
    if (le <= mid) sum += query(rt << 1, l, mid, le, re);
    if (re > mid) sum += query(rt << 1 | 1, mid + 1, r, le, re);
    return sum;
}

ll querypos(int rt, int l, int r, int pos) {
    if (pos == 0) return 0;
    if (l == r) return p[rt];
    int mid = l + r >> 1;
    if (pos <= mid) return querypos(rt << 1, l, mid, pos);
    return querypos(rt << 1 | 1, mid + 1, r, pos);
}

void updatea(int pos, int x) {
    ll tp = query(1, 1, n, 1, pos - 1);
    if (x - tp > b[pos]) update(1, 1, n, pos, x - tp);
    else update(1, 1, n, pos, b[pos]);
    a[pos] = x;
}

void updateb(int pos, int x) {
    ll tp = query(1, 1, n, 1, pos - 1);
    if (x + tp < a[pos]) update(1, 1, n, pos, a[pos] - tp);
    else update(1, 1, n, pos, x);
    b[pos] = x;
}

int main(int argc, char* argv[]) {
    int x, y;
    cin >> x >> y;
    vector<int> a(0);
    for (int i = 0; i < 1e8; ++i) {
        a.push_back(x);
        if (a.size() > 1e5) a.pop_back();
    }
    x = a[y] + y;
    cout << x << endl;
    return 0;
}

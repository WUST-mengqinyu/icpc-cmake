//
// Created by badcw on 2021/7/22.
//

#include "gen.h"

int main(int argc, char **argv) {
    registerGen(argc, argv, 1);
    int n = rnd.next(2, 5);
    int m = rnd.next(n * 2, 15);
    printf("1\n");// 1 case
    printf("%d %d\n", n, m);
    for (int i = 0; i < m; ++i) {
        int u = rnd.next(1, n);
        int v = rnd.next(1, n);
        int w = rnd.next(1, 10);
        printf("%d %d %d\n", u, v, w);
    }
    int k = rnd.next(1, n);
    printf("%d\n", k);
    for (int i = 1; i <= k; ++i) {
        int x = rnd.next(1, n);
        printf("%d ", x);
    }
    printf("\n");
    return 0;
}

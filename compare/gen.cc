//
// Created by badcw on 2021/7/22.
//

#include "gen.h"

int main(int argc, char **argv) {
    registerGen(argc, argv, 1);
    printf("%d %d\n", rnd.next(1, 1000), rnd.next(1, 1000));
    return 0;
}

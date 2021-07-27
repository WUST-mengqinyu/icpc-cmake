//
// Created by badcw on 2021/7/22.
//

#include <bits/stdc++.h>

using namespace std;

int main(int argc, char **argv) {
    auto start = chrono::system_clock::now(), end = start = chrono::system_clock::now();
    int dbg = 0;
    if (argc == 2) {
        puts("\033[31mnow start a debug model\n");
        dbg = 1;
    }
    for (int i = 1;; ++i) {
        printf("The result of No. %d Case is:  ", i);
        char gen[20];
        sprintf(gen, "./gen %d > ../data/data.in", i);
        printf("%s\n", gen);
        system(gen);
        if (!dbg) {
            system("./std < ../data/data.in > ../data/std.out");
            start = chrono::system_clock::now();
            system("./my < ../data/data.in > ../data/my.out");
            end = chrono::system_clock::now();
            if (system("diff ../data/std.out ../data/my.out")) {
                printf("\033[31mWrong Answer ");
                exit(1);
            }
        } else {
            system("./std < ../data/data.in");
            start = chrono::system_clock::now();
            system("./my < ../data/data.in");
            end = chrono::system_clock::now();
            getchar();
        }
        printf("\033[31mAccepted ");
        printf("\033[31m%fms\n", (double) ((end - start).count()) * std::chrono::microseconds::period::num /
                         std::chrono::milliseconds::period::den);
    }
    return 0;
}

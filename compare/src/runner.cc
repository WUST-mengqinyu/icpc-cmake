//
// Created by badcw on 2021/7/22.
//

#include <bits/stdc++.h>

using namespace std;

int main() {
    auto start = chrono::system_clock::now(), end = start = chrono::system_clock::now();
    for (int i = 1;; ++i) {
        printf("The result of No. %d Case is:  ", i);
        system("./gen > ../data/data.in");
        system("./std < ../data/data.in > ../data/std.out");
        start = chrono::system_clock::now();
        system("./my < ../data/data.in > ../data/my.out");
        end = chrono::system_clock::now();
        if (system("diff ../data/std.out ../data/my.out")) {
            printf("Wrong Answer ");
            exit(0);
        }
        printf("Accepted ");
        printf("%fms\n", (double) ((end - start).count()) * std::chrono::microseconds::period::num /
                         std::chrono::milliseconds::period::den);
    }
    return 0;
}

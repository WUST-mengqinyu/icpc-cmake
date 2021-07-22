//
// Created by badcw on 2021/7/22.
//

#include <benchmark/benchmark.h>

void SomeFunction() {
    ;
}

static void BM_SomeFunction(benchmark::State& state) {
    // Perform setup here
    for (auto _ : state) {
        // This code gets timed
        SomeFunction();
    }
}
// Register the function as a benchmark
BENCHMARK(BM_SomeFunction);
// Run the benchmark
BENCHMARK_MAIN();

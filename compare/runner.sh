#!/usr/bin/env bash
set -ex

#if you want to clean all
#bazel clean --expunge

# remove old things
rm -rf data/*.in data/*.out ||:
rm -rf bin/* ||:

# make bin for compile
mkdir -p bin

# determined gcc version(for mac user mostly)
CC=g++-11
command -v g++-11 >/dev/null 2>&1 || CC=g++
echo ${CC}

# build
bazel build src:std src:my && cp ../bazel-bin/compare/src/std bin && cp ../bazel-bin/compare/src/my bin
bazel build src:runner && cp ../bazel-bin/compare/src/runner bin
bazel build :gen && cp ../bazel-bin/compare/gen bin

# run
cd bin && ./runner dbg "$2"
#!/usr/bin/env bash
bazel build :main
./bazel-bin/main < data.in

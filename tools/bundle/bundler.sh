#!/usr/bin/env bash
# {source_file} {exec_name} {output_dir}
set -x

curdir=$(cd "$(dirname "$0")"; pwd)

export PATH=$PATH:${HOME}/.cargo/bin/

if ! command -v icpc-bundler >/dev/null 2>&1; then
  echo "icpc-bundler no exist, try install"
  git submodule update --init --recursive
  if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo no exist, try install"
    curl https://sh.rustup.rs -sSf | sh -s -- -y
  fi
  cargo install ${curdir}
fi

icpc-bundler gen --exec-name $2 --exec-path $1
clang -fkeep-system-includes -P -E $3/$2.cc > $3/$2_min.cc
exit 0
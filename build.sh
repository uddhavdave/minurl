#!/bin/bash -ex
#
docker pull clux/muslrust:stable
docker run -v $PWD:/volume --rm -t clux/muslrust:stable cargo build --release

docker build --progress=plain . -t minurl

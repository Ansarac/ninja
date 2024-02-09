#!/bin/bash

docker build -t ninja .

docker run --rm -it -p 7999:7999 --name=ninja \
        -e LOG=info \
        -v ~/.ninja:/root/.ninja \
        ninja run
#!/bin/bash

project=$(basename $(pwd))
bin=target/x86_64-unknown-linux-gnu/release/$project

echo "[build] With xargo"                                     && \
xargo build --target x86_64-unknown-linux-gnu --release       && \
echo "[build] strip symbols"                                  && \
strip $bin                                                    && \
exa -l $bin

[ $? -eq 0 -a "$1" = "-x" ] && echo "[run] $bin" && $bin

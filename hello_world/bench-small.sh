#!/bin/bash -e

if [ "$1" != "" ] && [ "$2" != "" ]; then
    LOG_FILE=$1
    URL=$2
    mkdir -p log/small
    echo ">>> Start Benchmark <<<" >> log/small/$LOG_FILE
    ab -n 100000 -c 100 $URL | tee -a log/small/$LOG_FILE
else
    echo "Usage: bench.sh LOG_FILE URL"
fi

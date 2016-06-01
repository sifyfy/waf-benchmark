#!/bin/bash -e

if [ "$1" != "" ] && [ "$2" != "" ]; then
    LOG_FILE=$1
    URL=$2
    echo ">>> Start Benchmark <<<" >> log/$LOG_FILE
    ab -n 1000000 -c 1000 $URL | tee -a log/$LOG_FILE
else
    echo "Usage: bench.sh LOG_FILE URL"
fi

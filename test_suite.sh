#!/bin/bash

#Test suite

if [ "$#" -ne 2 ]; then
    echo "Usage: ./test_suite.sh N_RUNS N_THREADS"
    exit 1
fi
runs=$1
threads=$2

mkdir -p results/
function run {
    #parameters: n, k, t
    ngrande=$((2**$1))
    ./ga $1 $2 $3 --log results/ga.$ngrande.$2.2.$3.log --runs $runs --threads $threads
    ./gp $1 $2 $3 --log results/gp.$ngrande.$2.2.$3.log --runs $runs --threads $threads
}

run 3 4 2
run 3 4 3
run 3 5 2
run 3 7 2
run 4 8 2
run 4 8 3
run 4 15 2
run 5 16 3
run 5 31 2
run 6 32 3
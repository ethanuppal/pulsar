#!/bin/bash
# REQUIRED INPUT $1 = name of build subdirectory
# REQUIRED INPUT $2 = name of top-level module 

set -x

BUILD_DIR="build"
N="$1"
MOD="$2"

if [[ "$(uname -s)" == "Darwin" ]]; then
    NUM_CORES=$(sysctl -n hw.logicalcpu)
    SED="sed -i ''"
else
    NUM_CORES=$(nproc)
    SED="sed -i"
fi

$SED "s/PULSAR_MAIN_MODULE/$MOD/g" "$BUILD_DIR/$N/sim_main.cpp"
cd "$BUILD_DIR/$N" && verilator \
    --cc --exe -sv --build -j "$NUM_CORES" \
    --top-module $MOD \
    -CFLAGS "-DHARNESS -DPULSAR_VERILATOR_TEST -I../../../phony" \
    sim_main.cpp "$N.sv" \
    && "obj_dir/V$MOD"

N			:= _
BUILD_DIR	:= build
PULSAR_MAIN	:= _pulsar_Smain_q_q
LOC			:= tests/calyx-verilog


ifeq ($(shell uname -s),Darwin)
NUM_CORES := $(shell sysctl -n hw.logicalcpu)
else
NUM_CORES := $(shell nproc)
endif

# rest of your Makefile
.PHONY: test
test:
	make clean
	if [ "$(N)" = "_" ]; then \
        echo "error: please supply a name via the 'N' define"; \
        exit 1; \
    fi
	cd ../.. && make
	cd ../.. && ./main $(LOC)/$(N).plsr 2>/dev/null 1>$(LOC)/$(BUILD_DIR)/$(N).sv
	cat harness/prefix.h harness/test.h harness/harness.cpp $(N).cpp > $(BUILD_DIR)/sim_main.cpp
	verilator \
 		--cc --exe -sv --build -j $(NUM_CORES) \
  		--top-module $(PULSAR_MAIN) \
		-CFLAGS "-DHARNESS -DPULSAR_VERILATOR_TEST -I../phony" \
		 $(BUILD_DIR)/sim_main.cpp $(BUILD_DIR)/$(N).sv
	obj_dir/V_pulsar_Smain_q_q
	make clean

.PHONY: clean
	find $(BUILD_DIR) -type f ! -name '.gitkeep' -delete
	rm -rf obj_dir

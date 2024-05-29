N			:= _
BUILD_DIR	:= build
PULSAR_MAIN	:= _pulsar_Smain_q_q
LOC			:= tests/calyx-verilog


ifeq ($(shell uname -s),Darwin)
NUM_CORES := $(shell sysctl -n hw.logicalcpu)
else
NUM_CORES := $(shell nproc)
endif

.PHONY: test
test:
	make clean N=$(N)
	if [ "$(N)" = "_" ]; then \
        echo "error: please supply a name via the 'N' define"; \
        exit 1; \
    fi
	mkdir -p $(BUILD_DIR)/$(N)
	cd ../.. && make
	cd ../.. && ./main $(LOC)/$(N).plsr 2>/dev/null 1>$(LOC)/$(BUILD_DIR)/$(N)/$(N).sv
	cat harness/prefix.h harness/test.h harness/harness.cpp $(N).cpp > $(BUILD_DIR)/$(N)/sim_main.cpp
	cd $(BUILD_DIR)/$(N) && verilator \
 		--cc --exe -sv --build -j $(NUM_CORES) \
  		--top-module $(PULSAR_MAIN) \
		-CFLAGS "-DHARNESS -DPULSAR_VERILATOR_TEST -I../../../phony" \
		sim_main.cpp $(N).sv
	$(BUILD_DIR)/$(N)/obj_dir/V_pulsar_Smain_q_q
	make clean N=$(N)

.PHONY: clean
clean:
	rm -rf ./$(BUILD_DIR)/$(N)

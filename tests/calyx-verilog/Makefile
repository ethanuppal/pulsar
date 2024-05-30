N			:= _
BUILD_DIR	:= build
LOC			:= tests/calyx-verilog

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
	chmod +x harness/invoke.bash
	harness/invoke.bash $(N) `grep -m 1 -o "_pulsar_Smain[^ \(]*" $(BUILD_DIR)/$(N)/$(N).sv | xargs`
	make clean N=$(N)

.PHONY: clean
clean:
	rm -rf ./$(BUILD_DIR)/$(N)

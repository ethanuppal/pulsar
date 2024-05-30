// Copyright (C) 2024 Ethan Uppal. All rights reserved.
#ifdef PULSAR_VERILATOR_TEST
    #include <iostream>

void PulsarMain::cycle() {
    mod->clk = !mod->clk;
    mod->eval();
}
void PulsarMain::pump() {
    for (int i = 0; i < 10; i++) {
        cycle();
    }
}
void PulsarMain::reset() {
    mod->reset = 1;
    pump();
    mod->reset = 0;
    pump();
}
void PulsarMain::go() {
    mod->go = 1;
    while (!mod->done) {
        cycle();
    }
    mod->go = 0;
    pump();
}

int test(PulsarMain main);

int main(int argc, char** argv) {
    Verilated::commandArgs(argc, argv);
    VPULSAR_MAIN_MODULE* mod = new VPULSAR_MAIN_MODULE;
    PulsarMain main;
    main.mod = mod;
    int exit_code = test(main);
    if (exit_code == 0) {
        std::cout << "test passed!" << '\n';
    }
    delete mod;
    exit(exit_code);
}
#endif

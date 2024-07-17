//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.
#ifdef PULSAR_VERILATOR_TEST
    #include <iostream>

void PulsarMain::cycle() {
    mod->clk = 0;
    mod->eval();
    mod->clk = 1;
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
    cycle();
}

    #ifdef __linux__
// linux hack for CI?
// https://veripool.org/guide/latest/faq.html#why-do-i-get-undefined-reference-to-sc-time-stamp
// likely not sustainable
double sc_time_stamp() {
    return 0;
}
    #endif

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

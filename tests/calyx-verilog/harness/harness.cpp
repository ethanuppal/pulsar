// Copyright (C) 2024 Ethan Uppal. All rights reserved.
#ifdef PULSAR_VERILATOR_TEST
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
    V_pulsar_Smain_q_q* mod = new V_pulsar_Smain_q_q;
    PulsarMain main;
    main.mod = mod;
    int exit_code = test(main);
    delete mod;
    exit(exit_code);
}
#endif

// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table implementation internals

#include "V_pulsar_Smain_q_q__pch.h"
#include "V_pulsar_Smain_q_q.h"
#include "V_pulsar_Smain_q_q___024root.h"

// FUNCTIONS
V_pulsar_Smain_q_q__Syms::~V_pulsar_Smain_q_q__Syms()
{
}

V_pulsar_Smain_q_q__Syms::V_pulsar_Smain_q_q__Syms(VerilatedContext* contextp, const char* namep, V_pulsar_Smain_q_q* modelp)
    : VerilatedSyms{contextp}
    // Setup internal state of the Syms class
    , __Vm_modelp{modelp}
    // Setup module instances
    , TOP{this, namep}
{
        // Check resources
        Verilated::stackCheck(213);
    // Configure time unit / time precision
    _vm_contextp__->timeunit(-12);
    _vm_contextp__->timeprecision(-12);
    // Setup each module's pointers to their submodules
    // Setup each module's pointer back to symbol table (for public functions)
    TOP.__Vconfigure(true);
    // Setup scopes
    __Vscope__pulsar_Smain_q_q__i3.configure(this, name(), "_pulsar_Smain_q_q.i3", "i3", -12, VerilatedScope::SCOPE_OTHER);
}

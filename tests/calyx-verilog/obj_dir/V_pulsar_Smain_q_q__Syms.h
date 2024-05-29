// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table internal header
//
// Internal details; most calling programs do not need this header,
// unless using verilator public meta comments.

#ifndef VERILATED_V_PULSAR_SMAIN_Q_Q__SYMS_H_
#define VERILATED_V_PULSAR_SMAIN_Q_Q__SYMS_H_  // guard

#include "verilated.h"

// INCLUDE MODEL CLASS

#include "V_pulsar_Smain_q_q.h"

// INCLUDE MODULE CLASSES
#include "V_pulsar_Smain_q_q___024root.h"

// SYMS CLASS (contains all model state)
class alignas(VL_CACHE_LINE_BYTES)V_pulsar_Smain_q_q__Syms final : public VerilatedSyms {
  public:
    // INTERNAL STATE
    V_pulsar_Smain_q_q* const __Vm_modelp;
    VlDeleter __Vm_deleter;
    bool __Vm_didInit = false;

    // MODULE INSTANCE STATE
    V_pulsar_Smain_q_q___024root   TOP;

    // SCOPE NAMES
    VerilatedScope __Vscope__pulsar_Smain_q_q__i3;

    // CONSTRUCTORS
    V_pulsar_Smain_q_q__Syms(VerilatedContext* contextp, const char* namep, V_pulsar_Smain_q_q* modelp);
    ~V_pulsar_Smain_q_q__Syms();

    // METHODS
    const char* name() { return TOP.name(); }
};

#endif  // guard

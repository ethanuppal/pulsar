// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Model implementation (design independent parts)

#include "V_pulsar_Smain_q_q__pch.h"

//============================================================
// Constructors

V_pulsar_Smain_q_q::V_pulsar_Smain_q_q(VerilatedContext* _vcontextp__, const char* _vcname__)
    : VerilatedModel{*_vcontextp__}
    , vlSymsp{new V_pulsar_Smain_q_q__Syms(contextp(), _vcname__, this)}
    , clk{vlSymsp->TOP.clk}
    , go{vlSymsp->TOP.go}
    , reset{vlSymsp->TOP.reset}
    , done{vlSymsp->TOP.done}
    , arg0{vlSymsp->TOP.arg0}
    , ret{vlSymsp->TOP.ret}
    , rootp{&(vlSymsp->TOP)}
{
    // Register model with the context
    contextp()->addModel(this);
}

V_pulsar_Smain_q_q::V_pulsar_Smain_q_q(const char* _vcname__)
    : V_pulsar_Smain_q_q(Verilated::threadContextp(), _vcname__)
{
}

//============================================================
// Destructor

V_pulsar_Smain_q_q::~V_pulsar_Smain_q_q() {
    delete vlSymsp;
}

//============================================================
// Evaluation function

#ifdef VL_DEBUG
void V_pulsar_Smain_q_q___024root___eval_debug_assertions(V_pulsar_Smain_q_q___024root* vlSelf);
#endif  // VL_DEBUG
void V_pulsar_Smain_q_q___024root___eval_static(V_pulsar_Smain_q_q___024root* vlSelf);
void V_pulsar_Smain_q_q___024root___eval_initial(V_pulsar_Smain_q_q___024root* vlSelf);
void V_pulsar_Smain_q_q___024root___eval_settle(V_pulsar_Smain_q_q___024root* vlSelf);
void V_pulsar_Smain_q_q___024root___eval(V_pulsar_Smain_q_q___024root* vlSelf);

void V_pulsar_Smain_q_q::eval_step() {
    VL_DEBUG_IF(VL_DBG_MSGF("+++++TOP Evaluate V_pulsar_Smain_q_q::eval_step\n"); );
#ifdef VL_DEBUG
    // Debug assertions
    V_pulsar_Smain_q_q___024root___eval_debug_assertions(&(vlSymsp->TOP));
#endif  // VL_DEBUG
    vlSymsp->__Vm_deleter.deleteAll();
    if (VL_UNLIKELY(!vlSymsp->__Vm_didInit)) {
        vlSymsp->__Vm_didInit = true;
        VL_DEBUG_IF(VL_DBG_MSGF("+ Initial\n"););
        V_pulsar_Smain_q_q___024root___eval_static(&(vlSymsp->TOP));
        V_pulsar_Smain_q_q___024root___eval_initial(&(vlSymsp->TOP));
        V_pulsar_Smain_q_q___024root___eval_settle(&(vlSymsp->TOP));
    }
    VL_DEBUG_IF(VL_DBG_MSGF("+ Eval\n"););
    V_pulsar_Smain_q_q___024root___eval(&(vlSymsp->TOP));
    // Evaluate cleanup
    Verilated::endOfEval(vlSymsp->__Vm_evalMsgQp);
}

//============================================================
// Events and timing
bool V_pulsar_Smain_q_q::eventsPending() { return false; }

uint64_t V_pulsar_Smain_q_q::nextTimeSlot() {
    VL_FATAL_MT(__FILE__, __LINE__, "", "%Error: No delays in the design");
    return 0;
}

//============================================================
// Utilities

const char* V_pulsar_Smain_q_q::name() const {
    return vlSymsp->name();
}

//============================================================
// Invoke final blocks

void V_pulsar_Smain_q_q___024root___eval_final(V_pulsar_Smain_q_q___024root* vlSelf);

VL_ATTR_COLD void V_pulsar_Smain_q_q::final() {
    V_pulsar_Smain_q_q___024root___eval_final(&(vlSymsp->TOP));
}

//============================================================
// Implementations of abstract methods from VerilatedModel

const char* V_pulsar_Smain_q_q::hierName() const { return vlSymsp->name(); }
const char* V_pulsar_Smain_q_q::modelName() const { return "V_pulsar_Smain_q_q"; }
unsigned V_pulsar_Smain_q_q::threads() const { return 1; }
void V_pulsar_Smain_q_q::prepareClone() const { contextp()->prepareClone(); }
void V_pulsar_Smain_q_q::atClone() const {
    contextp()->threadPoolpOnClone();
}

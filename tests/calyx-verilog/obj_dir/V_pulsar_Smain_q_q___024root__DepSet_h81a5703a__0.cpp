// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See V_pulsar_Smain_q_q.h for the primary calling header

#include "V_pulsar_Smain_q_q__pch.h"
#include "V_pulsar_Smain_q_q___024root.h"

void V_pulsar_Smain_q_q___024root___ico_sequent__TOP__0(V_pulsar_Smain_q_q___024root* vlSelf);

void V_pulsar_Smain_q_q___024root___eval_ico(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_ico\n"); );
    // Body
    if ((1ULL & vlSelf->__VicoTriggered.word(0U))) {
        V_pulsar_Smain_q_q___024root___ico_sequent__TOP__0(vlSelf);
    }
}

void V_pulsar_Smain_q_q___024root___eval_triggers__ico(V_pulsar_Smain_q_q___024root* vlSelf);

bool V_pulsar_Smain_q_q___024root___eval_phase__ico(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_phase__ico\n"); );
    // Init
    CData/*0:0*/ __VicoExecute;
    // Body
    V_pulsar_Smain_q_q___024root___eval_triggers__ico(vlSelf);
    __VicoExecute = vlSelf->__VicoTriggered.any();
    if (__VicoExecute) {
        V_pulsar_Smain_q_q___024root___eval_ico(vlSelf);
    }
    return (__VicoExecute);
}

void V_pulsar_Smain_q_q___024root___eval_act(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_act\n"); );
}

void V_pulsar_Smain_q_q___024root___nba_sequent__TOP__0(V_pulsar_Smain_q_q___024root* vlSelf);

void V_pulsar_Smain_q_q___024root___eval_nba(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_nba\n"); );
    // Body
    if ((1ULL & vlSelf->__VnbaTriggered.word(0U))) {
        V_pulsar_Smain_q_q___024root___nba_sequent__TOP__0(vlSelf);
    }
}

void V_pulsar_Smain_q_q___024root___eval_triggers__act(V_pulsar_Smain_q_q___024root* vlSelf);

bool V_pulsar_Smain_q_q___024root___eval_phase__act(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_phase__act\n"); );
    // Init
    VlTriggerVec<1> __VpreTriggered;
    CData/*0:0*/ __VactExecute;
    // Body
    V_pulsar_Smain_q_q___024root___eval_triggers__act(vlSelf);
    __VactExecute = vlSelf->__VactTriggered.any();
    if (__VactExecute) {
        __VpreTriggered.andNot(vlSelf->__VactTriggered, vlSelf->__VnbaTriggered);
        vlSelf->__VnbaTriggered.thisOr(vlSelf->__VactTriggered);
        V_pulsar_Smain_q_q___024root___eval_act(vlSelf);
    }
    return (__VactExecute);
}

bool V_pulsar_Smain_q_q___024root___eval_phase__nba(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_phase__nba\n"); );
    // Init
    CData/*0:0*/ __VnbaExecute;
    // Body
    __VnbaExecute = vlSelf->__VnbaTriggered.any();
    if (__VnbaExecute) {
        V_pulsar_Smain_q_q___024root___eval_nba(vlSelf);
        vlSelf->__VnbaTriggered.clear();
    }
    return (__VnbaExecute);
}

#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__ico(V_pulsar_Smain_q_q___024root* vlSelf);
#endif  // VL_DEBUG
#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__nba(V_pulsar_Smain_q_q___024root* vlSelf);
#endif  // VL_DEBUG
#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__act(V_pulsar_Smain_q_q___024root* vlSelf);
#endif  // VL_DEBUG

void V_pulsar_Smain_q_q___024root___eval(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval\n"); );
    // Init
    IData/*31:0*/ __VicoIterCount;
    CData/*0:0*/ __VicoContinue;
    IData/*31:0*/ __VnbaIterCount;
    CData/*0:0*/ __VnbaContinue;
    // Body
    __VicoIterCount = 0U;
    vlSelf->__VicoFirstIteration = 1U;
    __VicoContinue = 1U;
    while (__VicoContinue) {
        if (VL_UNLIKELY((0x64U < __VicoIterCount))) {
#ifdef VL_DEBUG
            V_pulsar_Smain_q_q___024root___dump_triggers__ico(vlSelf);
#endif
            VL_FATAL_MT("build/twice.sv", 1571, "", "Input combinational region did not converge.");
        }
        __VicoIterCount = ((IData)(1U) + __VicoIterCount);
        __VicoContinue = 0U;
        if (V_pulsar_Smain_q_q___024root___eval_phase__ico(vlSelf)) {
            __VicoContinue = 1U;
        }
        vlSelf->__VicoFirstIteration = 0U;
    }
    __VnbaIterCount = 0U;
    __VnbaContinue = 1U;
    while (__VnbaContinue) {
        if (VL_UNLIKELY((0x64U < __VnbaIterCount))) {
#ifdef VL_DEBUG
            V_pulsar_Smain_q_q___024root___dump_triggers__nba(vlSelf);
#endif
            VL_FATAL_MT("build/twice.sv", 1571, "", "NBA region did not converge.");
        }
        __VnbaIterCount = ((IData)(1U) + __VnbaIterCount);
        __VnbaContinue = 0U;
        vlSelf->__VactIterCount = 0U;
        vlSelf->__VactContinue = 1U;
        while (vlSelf->__VactContinue) {
            if (VL_UNLIKELY((0x64U < vlSelf->__VactIterCount))) {
#ifdef VL_DEBUG
                V_pulsar_Smain_q_q___024root___dump_triggers__act(vlSelf);
#endif
                VL_FATAL_MT("build/twice.sv", 1571, "", "Active region did not converge.");
            }
            vlSelf->__VactIterCount = ((IData)(1U) 
                                       + vlSelf->__VactIterCount);
            vlSelf->__VactContinue = 0U;
            if (V_pulsar_Smain_q_q___024root___eval_phase__act(vlSelf)) {
                vlSelf->__VactContinue = 1U;
            }
        }
        if (V_pulsar_Smain_q_q___024root___eval_phase__nba(vlSelf)) {
            __VnbaContinue = 1U;
        }
    }
}

#ifdef VL_DEBUG
void V_pulsar_Smain_q_q___024root___eval_debug_assertions(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_debug_assertions\n"); );
    // Body
    if (VL_UNLIKELY((vlSelf->go & 0xfeU))) {
        Verilated::overWidthError("go");}
    if (VL_UNLIKELY((vlSelf->clk & 0xfeU))) {
        Verilated::overWidthError("clk");}
    if (VL_UNLIKELY((vlSelf->reset & 0xfeU))) {
        Verilated::overWidthError("reset");}
}
#endif  // VL_DEBUG

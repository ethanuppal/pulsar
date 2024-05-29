// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See V_pulsar_Smain_q_q.h for the primary calling header

#include "V_pulsar_Smain_q_q__pch.h"
#include "V_pulsar_Smain_q_q___024root.h"

VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___eval_static(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_static\n"); );
}

VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___eval_initial(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_initial\n"); );
    // Body
    vlSelf->__Vtrigprevexpr___TOP__clk__0 = vlSelf->clk;
}

VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___eval_final(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_final\n"); );
}

#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__stl(V_pulsar_Smain_q_q___024root* vlSelf);
#endif  // VL_DEBUG
VL_ATTR_COLD bool V_pulsar_Smain_q_q___024root___eval_phase__stl(V_pulsar_Smain_q_q___024root* vlSelf);

VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___eval_settle(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_settle\n"); );
    // Init
    IData/*31:0*/ __VstlIterCount;
    CData/*0:0*/ __VstlContinue;
    // Body
    __VstlIterCount = 0U;
    vlSelf->__VstlFirstIteration = 1U;
    __VstlContinue = 1U;
    while (__VstlContinue) {
        if (VL_UNLIKELY((0x64U < __VstlIterCount))) {
#ifdef VL_DEBUG
            V_pulsar_Smain_q_q___024root___dump_triggers__stl(vlSelf);
#endif
            VL_FATAL_MT("build/twice.sv", 1571, "", "Settle region did not converge.");
        }
        __VstlIterCount = ((IData)(1U) + __VstlIterCount);
        __VstlContinue = 0U;
        if (V_pulsar_Smain_q_q___024root___eval_phase__stl(vlSelf)) {
            __VstlContinue = 1U;
        }
        vlSelf->__VstlFirstIteration = 0U;
    }
}

#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__stl(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___dump_triggers__stl\n"); );
    // Body
    if ((1U & (~ vlSelf->__VstlTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelf->__VstlTriggered.word(0U))) {
        VL_DBG_MSGF("         'stl' region trigger index 0 is active: Internal 'stl' trigger - first iteration\n");
    }
}
#endif  // VL_DEBUG

VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___stl_sequent__TOP__0(V_pulsar_Smain_q_q___024root* vlSelf);

VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___eval_stl(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_stl\n"); );
    // Body
    if ((1ULL & vlSelf->__VstlTriggered.word(0U))) {
        V_pulsar_Smain_q_q___024root___stl_sequent__TOP__0(vlSelf);
    }
}

VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___eval_triggers__stl(V_pulsar_Smain_q_q___024root* vlSelf);

VL_ATTR_COLD bool V_pulsar_Smain_q_q___024root___eval_phase__stl(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_phase__stl\n"); );
    // Init
    CData/*0:0*/ __VstlExecute;
    // Body
    V_pulsar_Smain_q_q___024root___eval_triggers__stl(vlSelf);
    __VstlExecute = vlSelf->__VstlTriggered.any();
    if (__VstlExecute) {
        V_pulsar_Smain_q_q___024root___eval_stl(vlSelf);
    }
    return (__VstlExecute);
}

#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__ico(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___dump_triggers__ico\n"); );
    // Body
    if ((1U & (~ vlSelf->__VicoTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelf->__VicoTriggered.word(0U))) {
        VL_DBG_MSGF("         'ico' region trigger index 0 is active: Internal 'ico' trigger - first iteration\n");
    }
}
#endif  // VL_DEBUG

#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__act(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___dump_triggers__act\n"); );
    // Body
    if ((1U & (~ vlSelf->__VactTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelf->__VactTriggered.word(0U))) {
        VL_DBG_MSGF("         'act' region trigger index 0 is active: @(posedge clk)\n");
    }
}
#endif  // VL_DEBUG

#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__nba(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___dump_triggers__nba\n"); );
    // Body
    if ((1U & (~ vlSelf->__VnbaTriggered.any()))) {
        VL_DBG_MSGF("         No triggers active\n");
    }
    if ((1ULL & vlSelf->__VnbaTriggered.word(0U))) {
        VL_DBG_MSGF("         'nba' region trigger index 0 is active: @(posedge clk)\n");
    }
}
#endif  // VL_DEBUG

VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___ctor_var_reset(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___ctor_var_reset\n"); );
    // Body
    vlSelf->arg0 = VL_RAND_RESET_Q(64);
    vlSelf->ret = VL_RAND_RESET_Q(64);
    vlSelf->go = VL_RAND_RESET_I(1);
    vlSelf->clk = VL_RAND_RESET_I(1);
    vlSelf->reset = VL_RAND_RESET_I(1);
    vlSelf->done = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__t1_write_en = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__t1_out = VL_RAND_RESET_Q(64);
    vlSelf->_pulsar_Smain_q_q__DOT__i2_write_en = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__i2_out = VL_RAND_RESET_Q(64);
    vlSelf->_pulsar_Smain_q_q__DOT__i3_write_en = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__i5_write_en = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__i5_out = VL_RAND_RESET_Q(64);
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q_go = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__i7_write_en = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__i7_out = VL_RAND_RESET_Q(64);
    vlSelf->_pulsar_Smain_q_q__DOT__i8_write_en = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__i8_out = VL_RAND_RESET_Q(64);
    vlSelf->_pulsar_Smain_q_q__DOT__adder_out = VL_RAND_RESET_Q(64);
    vlSelf->_pulsar_Smain_q_q__DOT__mult_go = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__fsm_in = VL_RAND_RESET_I(5);
    vlSelf->_pulsar_Smain_q_q__DOT__fsm_out = VL_RAND_RESET_I(5);
    vlSelf->_pulsar_Smain_q_q__DOT__sig_reg_out = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT___guard10 = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT___guard30 = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT___guard74 = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT___guard263 = VL_RAND_RESET_I(1);
    for (int __Vi0 = 0; __Vi0 < 4; ++__Vi0) {
        vlSelf->_pulsar_Smain_q_q__DOT__i3__DOT__mem[__Vi0] = VL_RAND_RESET_Q(64);
    }
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__t0_write_en = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__t0_out = VL_RAND_RESET_Q(64);
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i0_write_en = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i0_out = VL_RAND_RESET_Q(64);
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i1_write_en = VL_RAND_RESET_I(1);
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i1_out = VL_RAND_RESET_Q(64);
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_in = VL_RAND_RESET_I(2);
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out = VL_RAND_RESET_I(2);
    vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__rtmp = VL_RAND_RESET_Q(64);
    vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__ltmp = VL_RAND_RESET_Q(64);
    VL_RAND_RESET_W(128, vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp);
    vlSelf->__VdfgRegularize_h6171c202_0_0 = VL_RAND_RESET_I(1);
    vlSelf->__VdfgRegularize_h6171c202_0_1 = VL_RAND_RESET_I(1);
    vlSelf->__VdfgRegularize_h6171c202_0_2 = VL_RAND_RESET_I(1);
    vlSelf->__VdfgRegularize_h6171c202_0_3 = VL_RAND_RESET_I(1);
    vlSelf->__VdfgRegularize_h6171c202_0_4 = VL_RAND_RESET_I(1);
    vlSelf->__Vtrigprevexpr___TOP__clk__0 = VL_RAND_RESET_I(1);
}

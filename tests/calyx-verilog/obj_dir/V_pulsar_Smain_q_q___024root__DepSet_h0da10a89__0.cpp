// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See V_pulsar_Smain_q_q.h for the primary calling header

#include "V_pulsar_Smain_q_q__pch.h"
#include "V_pulsar_Smain_q_q__Syms.h"
#include "V_pulsar_Smain_q_q___024root.h"

#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__ico(V_pulsar_Smain_q_q___024root* vlSelf);
#endif  // VL_DEBUG

void V_pulsar_Smain_q_q___024root___eval_triggers__ico(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_triggers__ico\n"); );
    // Body
    vlSelf->__VicoTriggered.set(0U, (IData)(vlSelf->__VicoFirstIteration));
#ifdef VL_DEBUG
    if (VL_UNLIKELY(vlSymsp->_vm_contextp__->debug())) {
        V_pulsar_Smain_q_q___024root___dump_triggers__ico(vlSelf);
    }
#endif
}

VL_INLINE_OPT void V_pulsar_Smain_q_q___024root___ico_sequent__TOP__0(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___ico_sequent__TOP__0\n"); );
    // Init
    CData/*0:0*/ _pulsar_Smain_q_q__DOT___guard6;
    _pulsar_Smain_q_q__DOT___guard6 = 0;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT___guard12;
    _pulsar_Smain_q_q__DOT___guard12 = 0;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3;
    _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3 = 0;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7;
    _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7 = 0;
    // Body
    _pulsar_Smain_q_q__DOT___guard6 = ((IData)(vlSelf->go) 
                                       & (0U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)));
    vlSelf->_pulsar_Smain_q_q__DOT__fsm_in = ((IData)(_pulsar_Smain_q_q__DOT___guard6)
                                               ? 1U
                                               : ((0x12U 
                                                   == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))
                                                   ? 0U
                                                   : 
                                                  (((0U 
                                                     != (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                    & (0x12U 
                                                       != (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)))
                                                    ? 
                                                   (0x1fU 
                                                    & ((IData)(1U) 
                                                       + (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)))
                                                    : 0U)));
    vlSelf->_pulsar_Smain_q_q__DOT___guard10 = ((IData)(_pulsar_Smain_q_q__DOT___guard6) 
                                                | ((1U 
                                                    <= (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                   & (0x13U 
                                                      > (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__t1_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & (0x12U 
                                                      == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)));
    vlSelf->_pulsar_Smain_q_q__DOT___guard74 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                & (IData)(_pulsar_Smain_q_q__DOT___guard6));
    vlSelf->_pulsar_Smain_q_q__DOT__i7_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & ((0xbU 
                                                       == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                      | (5U 
                                                         == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__i8_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & ((9U 
                                                       == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                      | (0xdU 
                                                         == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__mult_go = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                               & ((0xeU 
                                                   <= (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                  & (0x11U 
                                                     > (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__i2_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & ((IData)(_pulsar_Smain_q_q__DOT___guard6) 
                                                      | (0x11U 
                                                         == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__i3_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard263) 
                                                      | (IData)(_pulsar_Smain_q_q__DOT___guard6)));
    vlSelf->_pulsar_Smain_q_q__DOT__i5_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & (((4U 
                                                        == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                       | (8U 
                                                          == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))) 
                                                      | (IData)(vlSelf->__VdfgRegularize_h6171c202_0_4)));
    vlSelf->__VdfgRegularize_h6171c202_0_1 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                              & ((2U 
                                                  == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                 | (0xaU 
                                                    == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->__VdfgRegularize_h6171c202_0_2 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                              & ((3U 
                                                  == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                 | (0xcU 
                                                    == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->__VdfgRegularize_h6171c202_0_3 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                              & ((1U 
                                                  == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                 | (5U 
                                                    == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->__VdfgRegularize_h6171c202_0_0 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                              & ((4U 
                                                  == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                 | (IData)(_pulsar_Smain_q_q__DOT___guard6)));
    vlSelf->_pulsar_Smain_q_q__DOT___guard30 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                & (0xbU 
                                                   == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)));
    _pulsar_Smain_q_q__DOT___guard12 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                        & (9U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)));
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q_go 
        = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
           & ((5U <= (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
              & (8U > (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    if (VL_UNLIKELY((4ULL <= ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_0)
                               ? 0ULL : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_1)
                                          ? 2ULL : 
                                         ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_2)
                                           ? 3ULL : 
                                          ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_3)
                                            ? 1ULL : 0ULL))))))) {
        VL_WRITEF_NX("[%0t] %%Error: twice.sv:38: Assertion failed in %N_pulsar_Smain_q_q.i3: comb_mem_d1: Out of bounds access\naddr0: %0#\nSIZE: 4\n",0,
                     64,VL_TIME_UNITED_Q(1),-12,vlSymsp->name(),
                     64,((IData)(vlSelf->__VdfgRegularize_h6171c202_0_0)
                          ? 0ULL : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_1)
                                     ? 2ULL : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_2)
                                                ? 3ULL
                                                : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_3)
                                                    ? 1ULL
                                                    : 0ULL)))));
        VL_STOP_MT("build/twice.sv", 38, "");
    }
    vlSelf->_pulsar_Smain_q_q__DOT__adder_out = (((IData)(_pulsar_Smain_q_q__DOT___guard12)
                                                   ? vlSelf->_pulsar_Smain_q_q__DOT__i5_out
                                                   : 
                                                  (((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                    & (0xdU 
                                                       == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)))
                                                    ? vlSelf->_pulsar_Smain_q_q__DOT__i7_out
                                                    : 
                                                   ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard30)
                                                     ? vlSelf->_pulsar_Smain_q_q__DOT__i8_out
                                                     : 0ULL))) 
                                                 + 
                                                 (((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & ((0xbU 
                                                       == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                      | (0xdU 
                                                         == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))))
                                                   ? vlSelf->_pulsar_Smain_q_q__DOT__i5_out
                                                   : 
                                                  ((IData)(_pulsar_Smain_q_q__DOT___guard12)
                                                    ? vlSelf->_pulsar_Smain_q_q__DOT__i7_out
                                                    : 0ULL)));
    _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3 
        = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q_go) 
           & (0U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)));
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_in 
        = ((IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3)
            ? 1U : ((2U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out))
                     ? 0U : (((0U != (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)) 
                              & (2U != (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)))
                              ? (3U & ((IData)(1U) 
                                       + (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)))
                              : 0U)));
    _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7 
        = ((IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3) 
           | ((1U <= (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)) 
              & (3U > (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__t0_write_en 
        = ((IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7) 
           & (2U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)));
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i0_write_en 
        = ((IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7) 
           & (IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3));
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i1_write_en 
        = ((IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7) 
           & (1U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)));
}

#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__act(V_pulsar_Smain_q_q___024root* vlSelf);
#endif  // VL_DEBUG

void V_pulsar_Smain_q_q___024root___eval_triggers__act(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_triggers__act\n"); );
    // Body
    vlSelf->__VactTriggered.set(0U, ((IData)(vlSelf->clk) 
                                     & (~ (IData)(vlSelf->__Vtrigprevexpr___TOP__clk__0))));
    vlSelf->__Vtrigprevexpr___TOP__clk__0 = vlSelf->clk;
#ifdef VL_DEBUG
    if (VL_UNLIKELY(vlSymsp->_vm_contextp__->debug())) {
        V_pulsar_Smain_q_q___024root___dump_triggers__act(vlSelf);
    }
#endif
}

VL_INLINE_OPT void V_pulsar_Smain_q_q___024root___nba_sequent__TOP__0(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___nba_sequent__TOP__0\n"); );
    // Init
    CData/*0:0*/ _pulsar_Smain_q_q__DOT___guard6;
    _pulsar_Smain_q_q__DOT___guard6 = 0;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT___guard12;
    _pulsar_Smain_q_q__DOT___guard12 = 0;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3;
    _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3 = 0;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7;
    _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7 = 0;
    QData/*63:0*/ __VdlyVal___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0;
    __VdlyVal___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0 = 0;
    CData/*1:0*/ __VdlyDim0___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0;
    __VdlyDim0___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0 = 0;
    QData/*63:0*/ __Vdly___pulsar_Smain_q_q__DOT__i5_out;
    __Vdly___pulsar_Smain_q_q__DOT__i5_out = 0;
    VlWide<4>/*127:0*/ __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp;
    VL_ZERO_W(128, __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp);
    CData/*0:0*/ __VdlySet___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0;
    __VdlySet___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0 = 0;
    VlWide<4>/*127:0*/ __Vtemp_2;
    VlWide<4>/*127:0*/ __Vtemp_3;
    VlWide<4>/*127:0*/ __Vtemp_4;
    // Body
    __VdlySet___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0 = 0U;
    __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[0U] 
        = vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[0U];
    __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[1U] 
        = vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[1U];
    __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[2U] 
        = vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[2U];
    __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[3U] 
        = vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[3U];
    __Vdly___pulsar_Smain_q_q__DOT__i5_out = vlSelf->_pulsar_Smain_q_q__DOT__i5_out;
    if (((~ (IData)(vlSelf->reset)) & (IData)(vlSelf->_pulsar_Smain_q_q__DOT__i3_write_en))) {
        __VdlyVal___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0 
            = (((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                & (IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard263))
                ? 0ULL : ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard74)
                           ? 1ULL : 0ULL));
        __VdlyDim0___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0 
            = (3U & (IData)(((IData)(vlSelf->__VdfgRegularize_h6171c202_0_0)
                              ? 0ULL : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_1)
                                         ? 2ULL : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_2)
                                                    ? 3ULL
                                                    : 
                                                   ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_3)
                                                     ? 1ULL
                                                     : 0ULL))))));
        __VdlySet___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0 = 1U;
    }
    __Vtemp_2[0U] = (IData)(vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__ltmp);
    __Vtemp_2[1U] = (IData)((vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__ltmp 
                             >> 0x20U));
    __Vtemp_2[2U] = 0U;
    __Vtemp_2[3U] = 0U;
    __Vtemp_3[0U] = (IData)(vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__rtmp);
    __Vtemp_3[1U] = (IData)((vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__rtmp 
                             >> 0x20U));
    __Vtemp_3[2U] = 0U;
    __Vtemp_3[3U] = 0U;
    VL_MUL_W(4, __Vtemp_4, __Vtemp_2, __Vtemp_3);
    if (vlSelf->reset) {
        __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[0U] = 0U;
        __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[1U] = 0U;
        __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[2U] = 0U;
        __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[3U] = 0U;
        __Vdly___pulsar_Smain_q_q__DOT__i5_out = 0ULL;
        vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out = 0U;
        vlSelf->_pulsar_Smain_q_q__DOT__sig_reg_out = 0U;
        vlSelf->_pulsar_Smain_q_q__DOT__t1_out = 0ULL;
        vlSelf->_pulsar_Smain_q_q__DOT__i7_out = 0ULL;
        vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__ltmp = 0ULL;
        vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__rtmp = 0ULL;
        vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__t0_out = 0ULL;
        vlSelf->_pulsar_Smain_q_q__DOT__i8_out = 0ULL;
        vlSelf->_pulsar_Smain_q_q__DOT__i2_out = 0ULL;
        vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i1_out = 0ULL;
        vlSelf->_pulsar_Smain_q_q__DOT__fsm_out = 0U;
        vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i0_out = 0ULL;
    } else {
        if (vlSelf->_pulsar_Smain_q_q__DOT__mult_go) {
            __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[0U] 
                = __Vtemp_4[0U];
            __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[1U] 
                = __Vtemp_4[1U];
            __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[2U] 
                = __Vtemp_4[2U];
            __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[3U] 
                = __Vtemp_4[3U];
            vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__ltmp 
                = vlSelf->_pulsar_Smain_q_q__DOT__i8_out;
            vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__rtmp 
                = vlSelf->_pulsar_Smain_q_q__DOT__i2_out;
        } else {
            __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[0U] 
                = vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[0U];
            __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[1U] 
                = vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[1U];
            __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[2U] 
                = vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[2U];
            __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[3U] 
                = vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[3U];
            vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__ltmp = 0ULL;
            vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__rtmp = 0ULL;
        }
        if (vlSelf->_pulsar_Smain_q_q__DOT__i5_write_en) {
            __Vdly___pulsar_Smain_q_q__DOT__i5_out 
                = (((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                    & (8U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)))
                    ? vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__t0_out
                    : (((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                        & ((4U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                           | (IData)(vlSelf->__VdfgRegularize_h6171c202_0_4)))
                        ? vlSelf->_pulsar_Smain_q_q__DOT__i3__DOT__mem
                       [((IData)(vlSelf->__VdfgRegularize_h6171c202_0_0)
                          ? 0U : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_1)
                                   ? 2U : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_2)
                                            ? 3U : 
                                           ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_3)
                                             ? 1U : 0U))))]
                        : 0ULL));
        }
        vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out 
            = vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_in;
        if ((0U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))) {
            vlSelf->_pulsar_Smain_q_q__DOT__sig_reg_out 
                = vlSelf->go;
        }
        if (vlSelf->_pulsar_Smain_q_q__DOT__t1_write_en) {
            vlSelf->_pulsar_Smain_q_q__DOT__t1_out 
                = vlSelf->_pulsar_Smain_q_q__DOT__i2_out;
        }
        if (vlSelf->_pulsar_Smain_q_q__DOT__i7_write_en) {
            vlSelf->_pulsar_Smain_q_q__DOT__i7_out 
                = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard30)
                    ? vlSelf->_pulsar_Smain_q_q__DOT__adder_out
                    : (((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                        & (5U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)))
                        ? vlSelf->_pulsar_Smain_q_q__DOT__i3__DOT__mem
                       [((IData)(vlSelf->__VdfgRegularize_h6171c202_0_0)
                          ? 0U : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_1)
                                   ? 2U : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_2)
                                            ? 3U : 
                                           ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_3)
                                             ? 1U : 0U))))]
                        : 0ULL));
        }
        if (vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__t0_write_en) {
            vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__t0_out 
                = vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i1_out;
        }
        if (vlSelf->_pulsar_Smain_q_q__DOT__i8_write_en) {
            vlSelf->_pulsar_Smain_q_q__DOT__i8_out 
                = vlSelf->_pulsar_Smain_q_q__DOT__adder_out;
        }
        if (vlSelf->_pulsar_Smain_q_q__DOT__i2_write_en) {
            vlSelf->_pulsar_Smain_q_q__DOT__i2_out 
                = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard74)
                    ? vlSelf->arg0 : (((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                       & (0x11U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)))
                                       ? (((QData)((IData)(
                                                           vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[1U])) 
                                           << 0x20U) 
                                          | (QData)((IData)(
                                                            vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[0U])))
                                       : 0ULL));
        }
        if (vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i1_write_en) {
            vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i1_out 
                = (1ULL + vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i0_out);
        }
        vlSelf->_pulsar_Smain_q_q__DOT__fsm_out = vlSelf->_pulsar_Smain_q_q__DOT__fsm_in;
        if (vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i0_write_en) {
            vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i0_out 
                = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q_go)
                    ? vlSelf->_pulsar_Smain_q_q__DOT__i5_out
                    : 0ULL);
        }
    }
    if (__VdlySet___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0) {
        vlSelf->_pulsar_Smain_q_q__DOT__i3__DOT__mem[__VdlyDim0___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0] 
            = __VdlyVal___pulsar_Smain_q_q__DOT__i3__DOT__mem__v0;
    }
    vlSelf->ret = vlSelf->_pulsar_Smain_q_q__DOT__t1_out;
    vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[0U] 
        = __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[0U];
    vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[1U] 
        = __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[1U];
    vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[2U] 
        = __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[2U];
    vlSelf->_pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[3U] 
        = __Vdly___pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp[3U];
    vlSelf->_pulsar_Smain_q_q__DOT__i5_out = __Vdly___pulsar_Smain_q_q__DOT__i5_out;
    vlSelf->done = ((0U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                    & (IData)(vlSelf->_pulsar_Smain_q_q__DOT__sig_reg_out));
    vlSelf->_pulsar_Smain_q_q__DOT___guard263 = ((1U 
                                                  == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                 | ((2U 
                                                     == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                    | (3U 
                                                       == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->__VdfgRegularize_h6171c202_0_4 = ((0xaU 
                                               == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                              | (0xcU 
                                                 == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)));
    _pulsar_Smain_q_q__DOT___guard6 = ((IData)(vlSelf->go) 
                                       & (0U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)));
    vlSelf->_pulsar_Smain_q_q__DOT__fsm_in = ((IData)(_pulsar_Smain_q_q__DOT___guard6)
                                               ? 1U
                                               : ((0x12U 
                                                   == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))
                                                   ? 0U
                                                   : 
                                                  (((0U 
                                                     != (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                    & (0x12U 
                                                       != (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)))
                                                    ? 
                                                   (0x1fU 
                                                    & ((IData)(1U) 
                                                       + (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)))
                                                    : 0U)));
    vlSelf->_pulsar_Smain_q_q__DOT___guard10 = ((IData)(_pulsar_Smain_q_q__DOT___guard6) 
                                                | ((1U 
                                                    <= (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                   & (0x13U 
                                                      > (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__t1_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & (0x12U 
                                                      == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)));
    vlSelf->_pulsar_Smain_q_q__DOT___guard74 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                & (IData)(_pulsar_Smain_q_q__DOT___guard6));
    vlSelf->_pulsar_Smain_q_q__DOT__i7_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & ((0xbU 
                                                       == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                      | (5U 
                                                         == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__i8_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & ((9U 
                                                       == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                      | (0xdU 
                                                         == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__mult_go = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                               & ((0xeU 
                                                   <= (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                  & (0x11U 
                                                     > (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__i2_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & ((IData)(_pulsar_Smain_q_q__DOT___guard6) 
                                                      | (0x11U 
                                                         == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__i3_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard263) 
                                                      | (IData)(_pulsar_Smain_q_q__DOT___guard6)));
    vlSelf->_pulsar_Smain_q_q__DOT__i5_write_en = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & (((4U 
                                                        == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                       | (8U 
                                                          == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))) 
                                                      | (IData)(vlSelf->__VdfgRegularize_h6171c202_0_4)));
    vlSelf->__VdfgRegularize_h6171c202_0_1 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                              & ((2U 
                                                  == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                 | (0xaU 
                                                    == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->__VdfgRegularize_h6171c202_0_2 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                              & ((3U 
                                                  == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                 | (0xcU 
                                                    == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->__VdfgRegularize_h6171c202_0_3 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                              & ((1U 
                                                  == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                 | (5U 
                                                    == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    vlSelf->__VdfgRegularize_h6171c202_0_0 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                              & ((4U 
                                                  == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                 | (IData)(_pulsar_Smain_q_q__DOT___guard6)));
    vlSelf->_pulsar_Smain_q_q__DOT___guard30 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                & (0xbU 
                                                   == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)));
    _pulsar_Smain_q_q__DOT___guard12 = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                        & (9U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)));
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q_go 
        = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
           & ((5U <= (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
              & (8U > (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))));
    if (VL_UNLIKELY((4ULL <= ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_0)
                               ? 0ULL : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_1)
                                          ? 2ULL : 
                                         ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_2)
                                           ? 3ULL : 
                                          ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_3)
                                            ? 1ULL : 0ULL))))))) {
        VL_WRITEF_NX("[%0t] %%Error: twice.sv:38: Assertion failed in %N_pulsar_Smain_q_q.i3: comb_mem_d1: Out of bounds access\naddr0: %0#\nSIZE: 4\n",0,
                     64,VL_TIME_UNITED_Q(1),-12,vlSymsp->name(),
                     64,((IData)(vlSelf->__VdfgRegularize_h6171c202_0_0)
                          ? 0ULL : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_1)
                                     ? 2ULL : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_2)
                                                ? 3ULL
                                                : ((IData)(vlSelf->__VdfgRegularize_h6171c202_0_3)
                                                    ? 1ULL
                                                    : 0ULL)))));
        VL_STOP_MT("build/twice.sv", 38, "");
    }
    vlSelf->_pulsar_Smain_q_q__DOT__adder_out = (((IData)(_pulsar_Smain_q_q__DOT___guard12)
                                                   ? vlSelf->_pulsar_Smain_q_q__DOT__i5_out
                                                   : 
                                                  (((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                    & (0xdU 
                                                       == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)))
                                                    ? vlSelf->_pulsar_Smain_q_q__DOT__i7_out
                                                    : 
                                                   ((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard30)
                                                     ? vlSelf->_pulsar_Smain_q_q__DOT__i8_out
                                                     : 0ULL))) 
                                                 + 
                                                 (((IData)(vlSelf->_pulsar_Smain_q_q__DOT___guard10) 
                                                   & ((0xbU 
                                                       == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out)) 
                                                      | (0xdU 
                                                         == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__fsm_out))))
                                                   ? vlSelf->_pulsar_Smain_q_q__DOT__i5_out
                                                   : 
                                                  ((IData)(_pulsar_Smain_q_q__DOT___guard12)
                                                    ? vlSelf->_pulsar_Smain_q_q__DOT__i7_out
                                                    : 0ULL)));
    _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3 
        = ((IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q_go) 
           & (0U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)));
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_in 
        = ((IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3)
            ? 1U : ((2U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out))
                     ? 0U : (((0U != (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)) 
                              & (2U != (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)))
                              ? (3U & ((IData)(1U) 
                                       + (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)))
                              : 0U)));
    _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7 
        = ((IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3) 
           | ((1U <= (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)) 
              & (3U > (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out))));
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__t0_write_en 
        = ((IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7) 
           & (2U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)));
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i0_write_en 
        = ((IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7) 
           & (IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard3));
    vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i1_write_en 
        = ((IData)(_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT___guard7) 
           & (1U == (IData)(vlSelf->_pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out)));
}

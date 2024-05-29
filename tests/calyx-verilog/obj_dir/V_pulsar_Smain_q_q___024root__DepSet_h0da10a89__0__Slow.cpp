// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See V_pulsar_Smain_q_q.h for the primary calling header

#include "V_pulsar_Smain_q_q__pch.h"
#include "V_pulsar_Smain_q_q__Syms.h"
#include "V_pulsar_Smain_q_q___024root.h"

#ifdef VL_DEBUG
VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___dump_triggers__stl(V_pulsar_Smain_q_q___024root* vlSelf);
#endif  // VL_DEBUG

VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___eval_triggers__stl(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___eval_triggers__stl\n"); );
    // Body
    vlSelf->__VstlTriggered.set(0U, (IData)(vlSelf->__VstlFirstIteration));
#ifdef VL_DEBUG
    if (VL_UNLIKELY(vlSymsp->_vm_contextp__->debug())) {
        V_pulsar_Smain_q_q___024root___dump_triggers__stl(vlSelf);
    }
#endif
}

VL_ATTR_COLD void V_pulsar_Smain_q_q___024root___stl_sequent__TOP__0(V_pulsar_Smain_q_q___024root* vlSelf) {
    (void)vlSelf;  // Prevent unused variable warning
    V_pulsar_Smain_q_q__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    VL_DEBUG_IF(VL_DBG_MSGF("+    V_pulsar_Smain_q_q___024root___stl_sequent__TOP__0\n"); );
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
    vlSelf->ret = vlSelf->_pulsar_Smain_q_q__DOT__t1_out;
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

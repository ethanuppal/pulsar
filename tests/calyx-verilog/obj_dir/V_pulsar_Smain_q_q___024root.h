// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design internal header
// See V_pulsar_Smain_q_q.h for the primary calling header

#ifndef VERILATED_V_PULSAR_SMAIN_Q_Q___024ROOT_H_
#define VERILATED_V_PULSAR_SMAIN_Q_Q___024ROOT_H_  // guard

#include "verilated.h"


class V_pulsar_Smain_q_q__Syms;

class alignas(VL_CACHE_LINE_BYTES) V_pulsar_Smain_q_q___024root final : public VerilatedModule {
  public:

    // DESIGN SPECIFIC STATE
    VL_IN8(clk,0,0);
    VL_IN8(go,0,0);
    VL_IN8(reset,0,0);
    VL_OUT8(done,0,0);
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__t1_write_en;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__i2_write_en;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__i3_write_en;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__i5_write_en;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q_go;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__i7_write_en;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__i8_write_en;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__mult_go;
    CData/*4:0*/ _pulsar_Smain_q_q__DOT__fsm_in;
    CData/*4:0*/ _pulsar_Smain_q_q__DOT__fsm_out;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__sig_reg_out;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT___guard10;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT___guard30;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT___guard74;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT___guard263;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__t0_write_en;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i0_write_en;
    CData/*0:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i1_write_en;
    CData/*1:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_in;
    CData/*1:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__fsm_out;
    CData/*0:0*/ __VdfgRegularize_h6171c202_0_0;
    CData/*0:0*/ __VdfgRegularize_h6171c202_0_1;
    CData/*0:0*/ __VdfgRegularize_h6171c202_0_2;
    CData/*0:0*/ __VdfgRegularize_h6171c202_0_3;
    CData/*0:0*/ __VdfgRegularize_h6171c202_0_4;
    CData/*0:0*/ __VstlFirstIteration;
    CData/*0:0*/ __VicoFirstIteration;
    CData/*0:0*/ __Vtrigprevexpr___TOP__clk__0;
    CData/*0:0*/ __VactContinue;
    VlWide<4>/*127:0*/ _pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__out_tmp;
    IData/*31:0*/ __VactIterCount;
    VL_IN64(arg0,63,0);
    VL_OUT64(ret,63,0);
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__t1_out;
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__i2_out;
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__i5_out;
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__i7_out;
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__i8_out;
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__adder_out;
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__t0_out;
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i0_out;
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__call_pulsar_Sinc_q_q__DOT__i1_out;
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__rtmp;
    QData/*63:0*/ _pulsar_Smain_q_q__DOT__mult__DOT__comp__DOT__ltmp;
    VlUnpacked<QData/*63:0*/, 4> _pulsar_Smain_q_q__DOT__i3__DOT__mem;
    VlTriggerVec<1> __VstlTriggered;
    VlTriggerVec<1> __VicoTriggered;
    VlTriggerVec<1> __VactTriggered;
    VlTriggerVec<1> __VnbaTriggered;

    // INTERNAL VARIABLES
    V_pulsar_Smain_q_q__Syms* const vlSymsp;

    // CONSTRUCTORS
    V_pulsar_Smain_q_q___024root(V_pulsar_Smain_q_q__Syms* symsp, const char* v__name);
    ~V_pulsar_Smain_q_q___024root();
    VL_UNCOPYABLE(V_pulsar_Smain_q_q___024root);

    // INTERNAL METHODS
    void __Vconfigure(bool first);
};


#endif  // guard

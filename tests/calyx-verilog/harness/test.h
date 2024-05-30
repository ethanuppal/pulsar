// Copyright (C) 2024 Ethan Uppal. All rights reserved.
#include <cstdint>

struct PulsarMain {
    struct dummy {
        int64_t ret;
        int64_t arg0;
        int64_t arg1;
        int64_t arg2;
        int64_t arg3;
        int64_t arg4;
        int64_t arg5;
        int64_t arg6;
        int64_t arg7;
        int64_t arg8;
        int64_t arg9;
    };

#ifdef HARNESS
    VPULSAR_MAIN_MODULE* mod;
#else
    dummy mod[1];
#endif
    void cycle();
    void pump();
    void reset();
    void go();
};

#ifndef plsr_reset
    #define plsr_reset(plsr) (plsr).reset()
#endif

#ifndef plsr_go
    #define plsr_go(plsr) (plsr).go()
#endif

#ifndef plsr_arg
    #define plsr_arg(plsr, i, value) (plsr).mod->arg##i = value
#endif

#ifndef plsr_ret
    #define plsr_ret(plsr) (plsr).mod->ret
#endif

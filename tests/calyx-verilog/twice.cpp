#include "harness/test.h"
#include <iostream>
#include <ctime>

int test(PulsarMain plsr) {
    plsr_reset(plsr);
    srand48(time(NULL));
    for (int i = 0; i < 1000; i++) {
        unsigned int arg = rand();
        plsr_arg(plsr, 0, arg);
        plsr_go(plsr);
        int result = plsr_ret(plsr);
        if (result != arg * 2) {
            std::cout << "test failed: expected: " << (arg * 2)
                      << " but received: " << result << '\n';
        }
    }
    return 0;
}

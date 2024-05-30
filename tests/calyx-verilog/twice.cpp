#include "harness/test.h"
#include <iostream>
#include <cstddef>
#include <ctime>
#include <cstdlib>

int test(PulsarMain plsr) {
    plsr_reset(plsr);
    for (int i = 0; i < 1000; i++) {
        unsigned int seed = time(NULL);
        unsigned int arg = rand_r(&seed) % 1000;
        plsr_arg(plsr, 0, arg);
        plsr_go(plsr);
        int result = plsr_ret(plsr);
        if (result != arg * 2) {
            std::cout << "test failed: expected: " << (arg * 2)
                      << " but received: " << result << '\n';
            return 1;
        }
    }
    return 0;
}

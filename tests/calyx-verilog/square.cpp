#include "harness/test.h"
#include <iostream>
#include <cstddef>
#include <ctime>
#include <cstdlib>
#include <random>

int test(PulsarMain plsr) {
    std::mt19937 generator(time(NULL));
    std::uniform_int_distribution<> distribution(0, 999);
    plsr_reset(plsr);
    for (int i = 0; i < 1000; i++) {
        unsigned int arg = distribution(generator);
        if (arg < 0) {
            arg = -arg;
        }
        plsr_arg(plsr, 0, arg);
        plsr_go(plsr);
        int result = plsr_ret(plsr);
        if (result != arg * arg) {
            std::cout << "test failed: expected: " << (arg * arg)
                      << " but received: " << result << '\n';
            return 1;
        }
    }
    return 0;
}

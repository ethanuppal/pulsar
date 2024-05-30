#include "harness/test.h"
#include <iostream>
#include <ctime>
#include <random>

int test(PulsarMain plsr) {
    plsr_reset(plsr);
    std::mt19937 generator(time(NULL));
    std::uniform_int_distribution<> distribution(0, 999);
    for (int i = 0; i < 1000; i++) {
        unsigned int arg = distribution(generator);
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

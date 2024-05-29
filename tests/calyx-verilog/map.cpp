#include "harness/test.h"
#include <iostream>
#include <cstddef>
#include <cstdlib>

int test(PulsarMain plsr) {
    plsr_reset(plsr);
    plsr_go(plsr);
    int result = plsr_ret(plsr);
    std::cout << "result: " << result << '\n';
    const int exp = 16;
    if (result != exp) {
        std::cout << "test failed: expected: " << exp
                  << " but received: " << result << '\n';
    }
    return 0;
}

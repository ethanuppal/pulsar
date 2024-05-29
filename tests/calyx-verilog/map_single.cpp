#include "harness/test.h"
#include <iostream>
#include <cstddef>
#include <cstdlib>

int test(PulsarMain plsr) {
    plsr_reset(plsr);
    plsr_go(plsr);
    // the program maps (+ 1) over a singleton array of 1 and returns the first
    // element in the array
    int result = plsr_ret(plsr);
    std::cout << "result: " << result << '\n';
    const int exp = 2;
    if (result != exp) {
        std::cout << "test failed: expected: " << exp
                  << " but received: " << result << '\n';
    }
    return 0;
}

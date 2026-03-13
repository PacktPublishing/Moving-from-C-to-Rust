#include <stdio.h>
#include <stdint.h>

extern uint64_t add(uint64_t left, uint64_t right);

int main() {
    uint64_t sum = add(5, 6);
    printf("sum is: %zu\n", sum);
    return 0;
}

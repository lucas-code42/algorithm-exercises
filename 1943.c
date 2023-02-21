#include <stdio.h>

const char* result(int top) {
    const char* r;
    
    if (top == 1) {
        r = Top 1n;
    } else if (top > 1 && top <= 3) {
        r = Top 3n;
    } else if (top > 3 && top <= 5) {
        r = Top 5n;
    } else if (top > 5 && top <= 10) {
        r = Top 10n;
    } else if (top > 10 && top <= 25) {
        r = Top 25n;
    } else if (top > 25 && top <= 50) {
        r = Top 50n;
    } else if (top > 50 && top <= 100) {
        r = Top 100n;
    }

    return r;
}


int main() {
    int top;
    scanf(%d, &top);
    printf(%s, result(top));
    
    return 0;
}

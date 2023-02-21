#include <stdio.h>

const char* result(int l , int r) {
    if (l > r) {
        return(-1n);
    } else if (l < r) {
        return(1n);
    } else {
        return(0n);
    }
}

int main() {
    int p1;
    int c1;

    int p2;
    int c2;

    scanf(%d, &p1);
    scanf(%d, &c1);
    scanf(%d, &p2);
    scanf(%d, &c2);

    int l = p1 * c1;
    int r = p2 * c2;
    
    printf(%s, result(l, r));
    return 0;
}



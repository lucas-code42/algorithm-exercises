#include <stdio.h>

int main() {
    int x;
    scanf(%d, &x);

    if (x >= 1 && x <= 35) {
        printf(Dn);
    } else if (x >= 36 && x <= 60) {
        printf(Cn);
    } else if (x >= 61 && x <= 85) {
        printf(Bn);
    } else if (x >= 86 && x <= 100) {
        printf(An);
    } else {
        printf(En);
    }
}

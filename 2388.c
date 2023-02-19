#include <stdio.h>

int main() {
    int n;
    scanf(%d, &n);

    double total;
    for (int i = 0; i < n; i++) {
        int x, y;
        scanf(%d, &x);
        scanf(%d, &y);

        total += x * y;
    }

    printf(%.0fn, total);

    return 0;
}

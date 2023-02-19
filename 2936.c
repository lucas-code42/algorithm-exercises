#include <stdio.h>

#define QUANTITY_PEOPLE 5
#define DONA_CHICA 225

#define CURUPIRRA 300 // 1
#define BOITATA 1500 // 2
#define BOTO 600 // 3
#define MAPINGUARI 1000 // 4
#define LARA 150 // 5


int main() {
    int total = DONA_CHICA;

    for (int i = 1; i <= QUANTITY_PEOPLE; i++) {
        int n;
        scanf(%d, &n);

        if (i == 1) {
            total += n * CURUPIRRA;
        } else if (i == 2) {
            total += n * BOITATA;
        } else if (i == 3) {
            total += n * BOTO;
        } else if (i == 4) {
            total += n * MAPINGUARI;
        } else if (i == 5) {
            total += n * LARA;
        }
    }

    printf(%dn, total);
    return 0;
}

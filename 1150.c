#include <stdio.h>

int calc(int init, int end) {
    int count = 0;
    int sum = 0;

    for(;;) {
        sum += init;
        count++;
        // printf("%d**", count);
        if (sum > end) {
            return count;
        }
        init++;
    }

}


int main() {
    int x;
    int y;
    scanf("%d", &x);
    do {
       scanf("%d", &y);
    } while (y <= x);
    // printf("PARAMS --> X %d ---> Y %d\n", x, y);
    
    int result = calc(x, y);
    printf("%d\n", result);
    return 0;
}
#include <stdio.h>

int calc(int grade, int avg) {
    return (avg * 2) - grade;
}

int main() {
    int grade, avg;
    scanf(%d%d, &grade, &avg);

    printf(%dn, calc(grade, avg));
    
    return 0;
}

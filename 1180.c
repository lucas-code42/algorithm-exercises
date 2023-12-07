#include <stdio.h>

int main()
{
    int n;
    scanf("%d", &n);

    int numbers[n];
    for (int i = 0; i < n; i++)
    {
        int tmp;
        scanf("%d", &tmp);
        numbers[i] = tmp;
    }

    int numbersLen = sizeof(numbers) / sizeof(numbers[0]);

    int lower = numbers[0];
    int pos = 0;
    for (int i = 0; i < numbersLen; i++)
    {
        if (numbers[i] < lower)
        {
            lower = numbers[i];
            pos = i;
        }
    }

    printf("Menor valor: %d\n", lower);
    printf("Posicao: %d\n", pos);
    return 0;
}
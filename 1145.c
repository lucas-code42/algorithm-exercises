#include <stdio.h>

int main()
{
    int x, y;
    scanf("%d %d", &x, &y);

    int n = y / x;
    int c = 1;

    for (int i = 0; i < n; i++)
    {
        if (c == y)
        {
            printf("%d", c);
            break;
        }

        for (int j = 0; j < x; j++)
        {

            if (j == x - 1)
            {
                printf("%d\n", c++);
            }
            else
            {
                printf("%d ", c++);
            }
        }
    }
    return 0;
}
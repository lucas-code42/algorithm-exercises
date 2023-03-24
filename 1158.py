iterate = int(input())
count_iterate = 0

while count_iterate < iterate:
    test = input().split(' ')
    y = int(test[0])
    x = int(test[1])

    if y % 2 == 0:
        # if the number is even
        temp_y = y + 1
        add = 0

        for i in range(x-1):
            temp_y += 2
            add += temp_y
        count_iterate += 1
        print(f'{add + y + 1}')
    else:
        # if the number is odd
        temp_y = y
        add = 0

        for i in range(x-1):
            temp_y += 2
            add += temp_y
        count_iterate += 1
        print(f'{add + y}')
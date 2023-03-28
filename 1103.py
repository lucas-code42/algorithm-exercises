while True:
    x = input().split( )
    
    h1 = int(x[0])
    m1 = int(x[1])

    h2 = int(x[2])
    m2 = int(x[3])

    if h1 == 0 and m1 == 0 and h2 == 0 and m2 == 0:
        break

    if h1 > h2 or h1 == h2 and m1 > m2:
        h2 += 24

    minutes = (h2*60 + m2) - (h1*60 + m1)
    print(minutes)

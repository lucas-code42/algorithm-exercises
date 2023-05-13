def main():
    x, y = None, None
    x = int(input())
    
    while True:
        y = int(input())
        if y > x:
            break
    print(calc(x, y))


    
def calc(init: int, end: int) -> int:
    # 3 + 4 + 5 + 6 + 7
    count = 0
    sum = 0
    while True:
        sum += init
        count += 1
        if sum > end:
            return count
        init += 1

if __name__ == "__main__":
    main()


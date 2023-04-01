def main():
    n = int(input())
    for i in range(n):
        v = float(input())

        c = 0
        while True:
            v = v / 2
            c += 1
            if v < 1:
                break

        print(f"{c} dias")
        

if __name__ == "__main__":
    main()

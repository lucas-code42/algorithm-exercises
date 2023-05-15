def main():
    n = int(input())
    for i in range(1, n+1):
        print(f"{i} {i*i} {(i*i*i)}")
        print(f"{i} {(i*i)+1} {(i*i*i)+1}")
    return


if __name__ == "__main__":
    main()
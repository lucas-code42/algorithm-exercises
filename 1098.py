import time


def main():
    I = 0
    J = 1
    S = 0.2
    first = True

    while True:
        if first:
            for i in range(3):
                print(f"I={I:.1f} J={J+i:.1f}")
            first = False
        else:
            for i in range(3):
                print(f"I={I:.1f} J={I+J:.1f}")
        I += S


if __name__ == "__main__":
    main()

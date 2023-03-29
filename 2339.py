
def main():
    inputs = input().split(" ")
    aluno = int(inputs[0])
    folhas = int(inputs[1])
    papel = int(inputs[2])

    if folhas < (aluno * papel):
        print("N")
    else:
        print("S")


if __name__ == "__main__":
    main()
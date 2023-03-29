
def main():
    inputs = input().split(" ")
    aluno = inputs[0]
    folhas = inputs[1]
    papel = inputs[2]

    if folhas < (aluno * papel):
        print("N")
    else:
        print("S")


if __name__ == "__main__":
    main()
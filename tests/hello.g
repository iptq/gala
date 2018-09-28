fn factorial(n: int): int =
    if n <= 1
        return 1
    else
        return n * factorial(n - 1)

fn main(args: String List): int =
    print("hello, world!\n")
    print(factorial(n))
    return 0

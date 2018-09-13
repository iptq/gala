use fmt

type Test: struct =
    x: int
    y: int

type Test2: enum =
    A: ()
    B: int

type Test3: trait =
    fn factorial(n: int): int

fn factorial(n: int): int =
    if n <= 1
        return 1
    else
        return n * factorial(n - 1)

fn main(args: string list): int =
    fmt.printf("hello, world!\n")

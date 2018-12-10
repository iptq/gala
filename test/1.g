extern puts: (string) -> int

struct Pair = 
    first: int
    second: int

fn factorial(n: int): int =
    let p = 1
    while n != 0:
        p = p * n
        n = n - 1
    return p

fn main(): int =
    puts("Hello, world!")
    return factorial(4)


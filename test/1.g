extern puts: int

fn factorial(n: int): int =
    let p = 1;
    while n != 0:
        p = p * n;
        n = n - 1;
    end;
    return p;
end

fn main(): int =
    puts("Hello, world!");
    return factorial(4);
end

extern puts: int

fn factorial_helper(x: int, y: int): int =
    if x == 0:
        return y;
    else:
        return factorial_helper(x - 1, y * x);
    end
end

fn factorial(n: int): int =
    return factorial_helper(n, 1);
end

fn main(): int =
    puts("Hello, world!");
    return factorial(4);
end

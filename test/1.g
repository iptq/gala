extern puts: int
extern printf: int

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
    x = 1;
    y = x + 2;
    z = x + y + 3;
    if x:
        u = 3;
    end;
    puts("Hello, world!");
    return factorial(4);
end

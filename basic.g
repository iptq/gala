type a = struct

fn main(): int =
  print "Hello, world!"
  let x = fibonacci(5)
  return x

fn fibonacci(n: int): int =
  return n * fibonacci(n - 1)

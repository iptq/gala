type a = struct

fn main(): int =
  print "Hello, world!"
  // let x = incr(5)
  let x = 1 + 1
  let y = 2 + 2
  print (x + y + 3)
  return x

fn incr(n: int): int =
  return n + 1

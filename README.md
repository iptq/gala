Gala
====

It's a language!

Running It
----------

```bash
make
```

Example
-------

This will print `"Hello, world!"` and also exit with status 24.

```
extern puts: (string) -> int

fn factorial(n: int): int =
    let p = 1
    while n != 0:
        p = p * n
        n = n - 1
    return p

fn main(): int =
    puts("Hello, world!")
    return factorial(4)
```

(waiting on custom lexer to remove semicolons and `end`s)

Checklist
---------

- [ ] Syntax
  - [x] Custom lexer
- [ ] Good Error Reporting
- [ ] Variables
  - [x] Declaration
  - [x] Assignment
- [ ] Control Flow
  - [x] If/Else
  - [x] While Loops
    - [ ] For Loops (depends on std iterators)
- [ ] Structs
  - [ ] Unions/Enums
- [x] Type Inference (in progress) 
  - [ ] First Class Functions
  - [ ] Generics
  - [ ] Type Classes
- [ ] Modules
  - [ ] Namespacing
- [ ] Future Stuff
  - [ ] Standard Library
  - [ ] Garbage Collection
  - [ ] Documentation (lol)
  - [ ] Proc Macros..??


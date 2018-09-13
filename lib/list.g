type 'a List: struct =
    size: int
    ptr: 'a ref
    cap: int

fn ('a List) push(self, item: 'a) =
    if self.size == self.cap
        self.realloc(self.cap * 2)
    self.ptr[self.size] = item

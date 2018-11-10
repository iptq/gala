all: a.out

.PHONY: clean

a.out: out.s
	gcc -o $@ -lc out.s

out.ll: test/1.g src/* Makefile
	cargo run -- test/1.g > $@

out.s: out.ll
	llc -relocation-model=pic -o $@ $<

clean:
	rm out.ll out.s out.o a.out

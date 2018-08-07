.PHONY: all clean

OCB_FLAGS = -use-ocamlfind -use-menhir -I src
OCB = corebuild $(OCB_FLAGS)

all: eval.byte 

clean:
	$(OCB) -clean

eval.byte: sanity
	$(OCB) $@

sanity:
	which menhir

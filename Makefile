.PHONY: all clean

OCB_FLAGS = -use-ocamlfind -use-menhir -I src
OCB = corebuild $(OCB_FLAGS)

all: cesk

clean:
	$(OCB) -clean

cesk: sanity
	$(OCB) cesk.byte

sanity:
	which menhir

.PHONY: a b

main_a: main_a.rs
	rustc $<

main_b: main_b.rs
	rustc $<

a: main_a
	./main_a

b: main_b
	./main_b

clean:
	rm -f main_a main_b

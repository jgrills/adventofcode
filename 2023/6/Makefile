.PHONY: a b

main_a: main_a.rs
	rustc $<

main_b: main_b.rs
	rustc $<

a: main_a
	./main_a input

b: main_b
	./main_b input

ae: main_a
	./main_a example

be: main_b
	./main_b example

clean:
	rm -f main_a main_b

addpush: clean
	git add .
	git commit
	git push

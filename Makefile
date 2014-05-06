brainruck:
	mkdir -p target
	rustc -O src/brainruck.rs --out-dir=target

clean:
	rm -r target/brainruck

.PHONY: clean

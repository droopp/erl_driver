.PHONY: build clean tests

build:
	@cd erl_driver/ && cargo build --release && cd -
	@gcc -o erl_driver.so -fpic -shared erl_driver.c -I ./include erl_driver/target/release/liberl_driver.a

tests:
	@cd test && erlc *.erl && cd -
	@cd test && RUST_LOG=trace erl -noshell -pa test -eval "eunit:test(erl_driver_test, [verbose])" -s init stop

clean:
	@cd erl_driver && cargo clean && cd -
	@rm -rf *.so
	@rm -rf test/*.beam

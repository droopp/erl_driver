.PHONY: build clean tests

build:
	@cd erl_driver/ && cargo build --release && cd -
	@gcc -o erl_driver.so -fpic -shared erl_driver.c -I ./include erl_driver/target/release/liberl_driver.a

tests:
	@cd test && erlc *.erl && cd -
	@cd test && erl +A 8 -noshell -pa test -eval "eunit:test(erl_driver_test, [verbose])" -s init stop

run:
	@cd test && erlc *.erl && cd -
	@cd test && erl +A 8 -pa test

clean:
	@cd erl_driver && cargo clean && cd -
	@rm -rf *.so
	@rm -rf test/*.beam

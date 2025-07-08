help: #: Print this help menu
	@echo "USAGE:\n"
	@cat $(MAKEFILE_LIST) \
		| grep '#:' \
		| grep -v 'grep' \
		| awk -F':' '{ OFS=":"; print $$1,$$3 }' \
		| column -t -s':'

test: #: Run the full test suite
	cargo test --workspace
	make -C harness test

test_%.debug: #: Run the test for '%' with debug flags
	make -C harness test_$*.debug

test_%: #: Run an individual test for '%'
	make -C harness $@.test

%.1: #: Display the man page for '%'
	man ./usr/share/man/man1/$*.1

%.skel: #: Make a skeleton for '%' (path)
	cargo new --bin $*
	cargo add --package $(notdir $*) clap --features derive
	echo "exit 1" \
		> harness/test_$(notdir $*).sh

clean: #: Remove any junk
	cargo clean
	make -C harness clean

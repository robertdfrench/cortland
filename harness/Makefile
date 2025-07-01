TESTS=$(wildcard test_*.sh)
TEST_TARGETS=$(patsubst %.sh,%.test,$(TESTS))

test: clean install banner $(TEST_TARGETS)

banner:
	@echo ""
	@echo "# Test Harness"

%.test: %.sh home/.dir
	$(info [$*])
	@cd home; PATH=../../target/release HOME=home /bin/sh ../$*.sh

%.debug: %.sh home/.dir
	$(warning [$*])
	@cd home; PATH=../../target/release HOME=home /bin/sh -ex ../$*.sh

home/.dir:
	mkdir -p home
	touch $@

install:
	cd ..; cargo build --release --workspace

clean:
	rm -rf  home

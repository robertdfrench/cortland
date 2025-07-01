test: install
	find . -type f -name "test_*.sh" \
		| xargs -n1 -Ix sh -c "echo x; PATH=../target/release /bin/sh x"

install:
	cd ..; cargo build --release --workspace

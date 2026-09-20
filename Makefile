install:
	cargo build --release
	mv target/release/password-generator /usr/local/bin/<PLACEHOLDER> <- insert name of the command here
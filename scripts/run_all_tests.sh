CYAN_COLOR="\033[0;36;1m"
RESET_FORMAT="\033[0m"

test_integer_info () {
	echo "\n${CYAN_COLOR}info${RESET_FORMAT}: running tests with $1-bit test integers..."
}

run_test () {
	test_integer_info "$1"
	RUSTFLAGS="--cfg test_int_bits=\"$1\"" cargo test int --lib --quiet --all-features
	if [ $? -ne 0 ]
	then
		exit 1
	fi
}

for bits in 128 64
do
	run_test $bits
done

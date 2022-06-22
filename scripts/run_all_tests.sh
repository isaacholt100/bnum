CYAN_COLOR="\033[0;36;1m"
RESET_FORMAT="\033[0m"

function digit_type_info() {
	echo "\n${CYAN_COLOR}info${RESET_FORMAT}: running bnum integer tests with \`$1\` as the \`Digit\` type..."
}

function test_integer_info() {
	echo "\n${CYAN_COLOR}info${RESET_FORMAT}: running tests with $1-bit test integers..."
}

function run_test() {
	export BNUM_TEST_INT_BITS=$1
	test_integer_info "$1"
	cargo test int --lib --quiet --features "$2"
	if [ $? -ne 0 ]
	then
		exit 1
	fi
}

digit_type_info "u64"

for bits in 64 128
do
	run_test $bits ""
done

digit_type_info "u8"

for bits in 8 16 32 64 128
do
	run_test $bits "u8_digit"
done
while true
do
	cargo test div --features "u8_digit"
	if [ $? -ne 0 ]
	then
		exit 1
	fi
done
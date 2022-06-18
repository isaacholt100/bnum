while true
do
	cargo test quickcheck_ --quiet
	clear
	if [ $? -ne 0 ]
	then
		exit 1
	fi
done
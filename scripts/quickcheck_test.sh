while true
do
	cargo test quickcheck_ --quiet
	clear && printf '\e[3J'
	if [ $? -ne 0 ]
	then
		exit 1
	fi
done
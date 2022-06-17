while true
do
    cargo test quickcheck_ --quiet
    if [[ x$? != x0 ]] ; then
        exit $?
    fi
	# clear && printf '\e[3J'
done
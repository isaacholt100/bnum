export QUICKCHECK_TESTS=100000
iters=0
while true
do
    "$@"
    if [ $? -ne 0 ]
    then
        exit 1
    fi
    clear && printf '\e[3J'
    ((iters=iters+1))
    echo "iterations: $iters"
done
#!/bin/sh

if (( $# != 1 )); then
    echo "Not provided binar!"
    exit 1
fi

cargo readobj --bin hello_word -- -file-headers 
openocd -f openocd.cfg -c "program $1 verify reset exit"
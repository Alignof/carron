#!/bin/bash

test_dir=/opt/riscv32/share/riscv-tests/isa/
test_kinds=(
    "rv32mi-p"
    "rv32si-p"
    "rv32ui-p"
    "rv32ui-v"
    "rv32um-p"
    "rv32um-v"
    "rv32ua-p"
    "rv32ua-v"
    "rv32uc-p"
    "rv32uc-v"
)

exit_status=0
for test_kind in ${test_kinds[@]}; do
    for test_name in `ls $test_dir | grep $test_kind | grep -v .dump`; do
        cargo r -- $test_dir$test_name > /dev/null 2>&1;
        result=$?;

        ESC=$(printf '\033');
        if [ $result = 1 ]; then
            echo "$test_name ${ESC}[32;1m ... passed ${ESC}[m"
        else
            echo "$test_name ${ESC}[31;1m ... failed ${ESC}[m";
            exit_status=1;
        fi

    done;
done;

exit $exit_status

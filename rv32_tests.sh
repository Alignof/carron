#!/bin/bash

test_dir=/opt/riscv32/share/riscv-tests/isa/
test_kinds=(
    "rv32ui-p"
    "rv32ui-v"
    "rv32uc-p"
    "rv32uc-v"
)

for test_kind in ${test_kinds[@]}; do
    for test_name in `ls $test_dir | grep $test_kind | grep -v .dump`; do
        if [ ${test_kind: -1} = "p" ]; then
            cargo r -- --break_point=0x80000044 --result_reg=3 $test_dir$test_name > /dev/null 2>&1;
        else 
            cargo r -- --break_point=0xffc02308 --result_reg=10 $test_dir$test_name > /dev/null 2>&1;
        fi
        result=$?;

        ESC=$(printf '\033');
        if [ $result = 1 ]; then
            echo "$test_name ${ESC}[32;1m ... passed ${ESC}[m"
        else
            echo "$test_name ${ESC}[31;1m ... failed ${ESC}[m"
        fi

    done;
done;

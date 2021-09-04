#!/bin/bash

test_dir=/opt/riscv32/share/riscv-tests/isa/
test_kind="rv32ui-p"

for test_name in `ls $test_dir | grep $test_kind | grep -v .dump`; do
    cargo r $test_dir$test_name > /dev/null 2>&1;
    result=$?;

	ESC=$(printf '\033');
	if [ $result = 1 ]; then
		echo "$test_name ${ESC}[32;1m ... passed ${ESC}[m"
	else
		echo "$test_name ${ESC}[31;1m ... failed ${ESC}[m"
	fi

done;

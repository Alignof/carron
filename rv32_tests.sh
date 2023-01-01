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
ESC=$(printf '\033');
p_filter="0x800000(3c|40|44|48)"
v_filter="(0xffc021(48|4c|50|54|58))|(0xffc023(04|08))"

diff_output() {
    if [ ${test_kind: -1} = "p" ]; then
        filter=$p_filter;
    else
        filter=$v_filter;
    fi;

    cargo r -- --isa=rv32 --loglv=info $test_dir$test_name 2> /dev/null |
        perl -ne 'print if /^pc: /' |
        perl -pe 's/pc: //' |
        perl -ne "print unless /${filter}/" > ./target/output;
    spike -l --isa=RV32IMAC $pk_path $test_dir$test_name 2>&1 |
        perl -pe 's/core.+: //' |
        perl -pe 's/^: //' |
        perl -pe 's/ \(0x.+$//' |
        perl -ne "print unless /${filter}/ or /^\s/ or /exception/" > ./target/expect;

    diff ./target/output ./target/expect
    if [ $? = 0 ]; then
        echo "$test_name ${ESC}[32;1m ... passed ${ESC}[m"
        rm -f ./target/output ./target/expect
    else
        echo "$test_name ${ESC}[31;1m ... failed ${ESC}[m";
        exit 1
        exit_status=1;
    fi;
}

exit_code() {
    cargo r -- --isa=rv32 $test_dir$test_name > /dev/null 2>&1;
    if [ $? = 1 ]; then
        echo "$test_name ${ESC}[32;1m ... passed ${ESC}[m"
    else
        echo "$test_name ${ESC}[31;1m ... failed ${ESC}[m";
        exit_status=1;
    fi;
}

flag=$1
exit_status=0
for test_kind in ${test_kinds[@]}; do
    for test_name in `ls $test_dir | grep $test_kind | grep -v .dump`; do
        if [ $(which spike 2> /dev/null) ] && [ "$flag" != "--exit_code" ]; then
            diff_output;
        else
            exit_code;
        fi
    done;
done;

exit $exit_status

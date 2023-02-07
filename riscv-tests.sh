#!/bin/bash

test_dir=/opt/riscv/riscv-tests/isa/
ESC=$(printf '\033');
exit_status=0

diff_output() {
    p_filter="0x(00000000)?800000(3c|40|44|48)"
    v_filter="(0x(00000000)?ffc021(48|4c|50|54|58))|(0xffc023(04|08))|0xffffffffffe022(b8|bc)"
    if [ ${test_kind: -1} = "p" ]; then
        filter=$p_filter;
    else
        filter=$v_filter;
    fi;

    timeout --foreground 3 cargo r --release -- --loglv=info $test_dir$test_name 2> /dev/null |
        perl -ne 'print if /^pc: /' |
        perl -pe 's/pc: //' |
        perl -ne "print unless /${filter}/" > ./target/output;
    spike -l --isa=${isa}IMAC $pk_path $test_dir$test_name 2>&1 |
        perl -pe 's/core.+: //' |
        perl -pe 's/^: //' |
        perl -pe 's/ \(0x.+$//' |
        perl -ne "print unless /${filter}/ or /^\s/ or /exception/ or /trigger action 0/" > ./target/expect;

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
    timeout --foreground 3 cargo r --release -- $test_dir$test_name > /dev/null 2>&1;
    if [ $? = 1 ]; then
        echo "$test_name ${ESC}[32;1m ... passed ${ESC}[m"
    else
        echo "$test_name ${ESC}[31;1m ... failed ${ESC}[m";
        exit_status=1;
    fi;
}

isa=$(echo $1 | perl -pe 's/--isa=//')
test_kinds=(
    "${isa}ui-p"
    "${isa}ui-v"
    "${isa}um-p"
    "${isa}um-v"
    "${isa}ua-p"
    "${isa}ua-v"
    "${isa}uc-p"
    "${isa}uc-v"
    "${isa}mi-p"
    "${isa}si-p"
)

excepts=(
    "rv64ui-p-ma_data"
    "rv64ui-v-ma_data"
)

cargo build --release
for test_kind in ${test_kinds[@]}; do
    for test_name in `ls $test_dir | grep $test_kind | grep -v .dump`; do
        if [[ ! "${excepts[@]}" =~ "${test_name}" ]]; then
            exit_code
            #diff_output
        fi
    done;
done;

exit $exit_status

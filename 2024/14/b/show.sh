#! /bin/bash

start=2300

while true; do
    clear
    echo time ${start}
    head -n 80 time_${start} | sed "s/\./ /g"
    read -n 1 ans
    echo ans: \'${ans}\'
    if [[ $ans == "." ]]; then
        (( ++start ))
    fi
    if [[ $ans == "," ]]; then
        (( --start ))
    fi
    if [[ $ans == "q" ]]; then
        echo quit ${start}
        exit 0
    fi
done

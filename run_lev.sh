#!/bin/bash
cargo run --release -- -t plagiarismbasic_lib/testfiles/cs-corpus/t/ -u plagiarismbasic_lib/testfiles/cs-corpus/ut/ -m lev -n 5 -s 1 --openhtml --cli --html

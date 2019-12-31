sudo perf record -g target/debug/plagiarism-basic -t plagiarismbasic_lib/testfiles/cs-corpus/t/ -u plagiarismbasic_lib/testfiles/cs-corpus/ut/ -m equal -n 10 -s 0 --html

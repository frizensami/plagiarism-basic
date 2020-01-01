cd plagiarismbasic_lib/ && cargo publish && echo "Sleeping to allow cargo publish for lib to take effect" && sleep 10 & cd ../plagiarism-basic && cargo publish && cd ..

#!/bin/bash

set -eu

rm -f tools/out/*.svg
rm -f tools/output/*.txt

cargo run --bin a < tools/in/0000.txt

file_num=`ls -p -U1 tools/output | grep -v / | wc -l`
cd tools
python make-svgs.py ${file_num}

cd ..
cargo run --bin movie ${file_num}

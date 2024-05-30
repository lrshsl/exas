#!/bin/sh

if [ $# -eq 0 ]; then
    echo "Usage: $0 <files>"
    exit 1
fi

if [ ! -f grammar/bnf.g4 ]; then
    echo "File grammar/bnf.g4 not found. Must be executed from the project root directory (where the Cargo.toml file is located)"
    exit 1
fi

for file in $@; do
    echo -e "<-< File $file, writing contents: >->\n<<<"
    cat $file
    echo -e ">>>>\n\n<-< End file contents >->"
    antlr4-parse grammar/bnf.g4 ast -gui $file
    echo -e "<-< End $file >->\n"
done


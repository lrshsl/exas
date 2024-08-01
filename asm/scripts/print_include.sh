#!/bin/sh


[[ $# -eq 1 ]] || { echo "Usage: $0 <input>"; exit 1; }

echo "#include \"$1.h\"" | gcc -E -dM -

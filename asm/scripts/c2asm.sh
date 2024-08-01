#!/bin/sh

scripts/c2asm_preprocessor.sh $@ | scripts/c2asm_octal.sh


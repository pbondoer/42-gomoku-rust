#!/bin/sh

red='\033[0;31m'
green='\033[0;32m'

files="$(find . -name '*.rs')"

for val in $files
do
	echo ${green}$val
	rustfmt $val
done
echo "${red}Formatted files with rustfmt done"

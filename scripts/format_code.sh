#!/bin/sh

files="$(find . -name '*.rs')"

for val in $files
do
	tput setaf 2
	echo $val
	rustfmt $val
done
tput setaf 1
echo "Formatted files with rustfmt done"

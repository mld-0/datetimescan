#!/usr/bin/env sh
#	{{{3
#   vim: set tabstop=4 modeline modelines=10:
#   vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
#	{{{2
set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

SCRIPTPATH="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
path_testfile_isodatetimes="$HOME/_data/textWithIsoDatetimes.txt"

#	validate: path_testfile_isodatetimes, SCRIPTPATH
#	{{{
if [[ ! -f "$path_testfile_isodatetimes" ]]; then
	echo "Failed to find path_testfile_isodatetimes=($path_testfile_isodatetimes)" > /dev/stderr
	exit 2
fi
if [[ ! -d "$SCRIPTPATH" ]]; then
	echo "Failed to find SCRIPTPATH=($SCRIPTPATH)" > /dev/stderr
	exit 2
fi
#	}}}

cmd_cargo="$HOME/.cargo/bin/cargo"
cmd_build=( $cmd_cargo build --release )
cmd_datetimescan="$SCRIPTPATH/target/debug/datetimescan"

main() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "warning, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	build_release
	test_scan
}

build_release() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "warning, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	cd "$SCRIPTPATH"
	${cmd_build[@]}
}

test_scan() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "warning, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local result_str=""
	local expected_str=""

	test_num=1
	test_cmd=( $cmd_datetimescan scan --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05-05T19:34:42+1000	1	0
2023-05-05T19:35:23+1000	2	0
2023-05-05T19:35:44+1000	3	10
2023-05-05T19:36:18+1000	4	0
2023-05-05T19:36:35+1000	5	0"
	assert_result

	echo "test_scan, DONE"
}

assert_result() {
	print_result
	if [[ ! "$result_str" == "$expected_str" ]]; then
		echo "$func_name, fail: $test_num\n"
		echo "$func_name, result_str=($result_str)"
		echo "$func_name, expected_str=($expected_str)"
		diff <( echo $result_str ) <( echo $expected_str )
		exit 2
	fi
}

print_result() {
	num=60
	prefix_str="$func_name: $test_num, result_str: "
	num_equals=$(($num - ${#prefix_str}))
	if ((num_equals < 1)); then num_equals=1; fi  
	output_str="${prefix_str}$(printf '=%.0s' $(seq 1 $num_equals))"
	echo "$output_str" > /dev/stderr
	echo "$result_str" > /dev/stderr
	printf '=%.0s' $(seq 1 $num) > /dev/stderr
	echo "" > /dev/stderr
}

main "$@"


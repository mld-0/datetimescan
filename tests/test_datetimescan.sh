#!/usr/bin/env zsh
#	vim-modelines:	{{{3
#   vim: set tabstop=4 modeline modelines=10:
#   vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
#	{{{2
#	Notes:
#	{{{
#	2023-05-20T21:10:59AEST instead of calling this from Rust - just write it in Rust?
#	2023-05-20T21:12:04AEST PROJECT_PATH is correct even when running script with `:w !zsh`?
#	}}}

set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

#set -x 

SCRIPTPATH="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
#	validate: SCRIPTPATH
#	{{{
if [[ ! -d "$SCRIPTPATH" ]]; then
	echo "Failed to find dir SCRIPTPATH=($SCRIPTPATH)" > /dev/stderr
	exit 2
fi
#	}}}

PROJECT_PATH=$( dirname "$SCRIPTPATH" )
path_testfile_isodatetimes="$PROJECT_PATH/tests/data/textWithIsoDatetimes-1.txt"
path_testfile_isodatetimes_2="$PROJECT_PATH/tests/data/textWithIsoDatetimes-2.txt"
path_testfile_empty="$PROJECT_PATH/tests/data/empty.txt"
#	validate: path_testfile_*, PROJECT_PATH
#	{{{
if [[ ! -f "$path_testfile_isodatetimes" ]]; then
	echo "Failed to find path_testfile_isodatetimes=($path_testfile_isodatetimes)" > /dev/stderr
	exit 2
fi
if [[ ! -f "$path_testfile_isodatetimes_2" ]]; then
	echo "Failed to find path_testfile_isodatetimes_2=($path_testfile_isodatetimes_2)" > /dev/stderr
	exit 2
fi
if [[ ! -f "$path_testfile_empty" ]]; then
	echo "Failed to find path_testfile_empty=($path_testfile_empty)" > /dev/stderr
	exit 2
fi
if [[ ! -d "$PROJECT_PATH" ]]; then
	echo "Failed to find PROJECT_PATH=($PROJECT_PATH)" > /dev/stderr
	exit 2
fi
#	}}}

cmd_cargo="$HOME/.cargo/bin/cargo"
cmd_build=( $cmd_cargo build --release )
cmd_datetimescan="$PROJECT_PATH/target/release/datetimescan"

flag_print_results=0
flag_exit_on_fail=0
failures_count=0
rust_log_level=''		#	set to 'trace' for full output

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
	if [[ ! -z "$rust_log_level" ]]; then
		export RUST_LOG="$rust_log_level"
	fi
	build_release
	test_scan
	#test_parse
	#test_filter
	test_count
	test_deltas
	test_splits
	test_sum
	#test_wpm
	if [[ $failures_count -gt 0 ]]; then
		echo "$func_name, failures_count=($failures_count)" > /dev/stderr
		exit 2
	fi
	echo "$func_name, DONE" > /dev/stderr
}

build_release() {
#	{{{
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
	echo cd "$PROJECT_PATH" > /dev/stderr
	echo ${cmd_build[@]} > /dev/stderr
	cd "$PROJECT_PATH"
	${cmd_build[@]}
	echo "$func_name, DONE" > /dev/stderr
}
#	}}}

test_scan() {
#	{{{
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
	local test_num=1

	test_cmd=( $cmd_datetimescan scan )
	result_str=$( echo "" | ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $cmd_datetimescan scan --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $cmd_datetimescan scan --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05-05T19:34:42+1000	1	0
2023-05-05T19:35:23+1000	2	0
2023-05-05T19:35:44+1000	3	10
2023-05-05T19:36:18+1000	4	0
2023-05-05T19:36:35+1000	5	0"
	assert_result

	test_cmd=( $cmd_datetimescan scan --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "2023-04-19T22:07:40AEST	1	10 2023-04-19T22:08:16AEST	2	0 2023-04-19T22:11:06AEST	4	10 2024-04-19T22:12:54AEST	6	0 2023-04-19T22:14:15AEST	10	0 2023-04-19T22:14:20AEST	12	6 2023-04-19T22:15:31AEST	13	0 2023-04-19T22:15:58AEST	15	9 2023-04-19T22:16:28AEST	16	0 2023-04-19T22:16:51AEST	18	0 2023-04-19T22:18:19AEST	19	0 2023-04-19T22:19:41AEST	21	6 2023-04-19T22:19:55AEST	22	0 2023-04-19T22:21:07AEST	24	9 2023-04-19T22:20:05AEST	25	6 2023-04-19T22:23:14AEST	26	0 2023-04-19T22:26:27AEST	28	0 2023-04-19T22:26:44AEST	29	0 2023-04-19T22:27:55AEST	31	0 2023-04-19T22:29:20AEST	32	0 2023-04-19T22:29:43AEST	34	9 2023-04-19T22:30:08AEST	35	0 2023-04-19T22:30:35AEST	37	0 2023-04-19T22:31:13AEST	38	0 2023-04-19T22:32:48AEST	39	0 2023-04-19T22:32:56AEST	41	0 2023-04-19T22:33:10AEST	42	10 2023-04-19T22:33:51AEST	43	6 2023-04-19T22:34:40AEST	44	0 2023-04-19T22:35:03AEST	46	6 2023-04-19T22:35:42AEST	47	0 2023-04-19T22:37:19AEST	49	9 2023-04-19T22:38:07AEST	50	0 2023-04-19T22:38:14AEST	52	0 2023-04-19T22:38:40AEST	53	0 2023-04-19T22:39:01AEST	55	0 2023-04-19T22:39:32AEST	56	0 2023-04-19T22:40:00AEST	59	0 2023-04-19T22:40:05AEST	60	0 2023-04-19T22:40:31AEST	61	0 2023-04-19T23:21:37AEST	63	0 2023-04-19T23:21:47AEST	64	0 2023-04-19T23:22:02AEST	67	0 2023-04-19T23:22:36AEST	68	0 2023-04-19T23:23:04AEST	69	0 2023-04-19T23:28:52AEST	71	10 2023-04-19T23:29:19AEST	73	7 2023-04-19T23:29:34AEST	74	0 2023-04-19T23:29:49AEST	76	6 2023-04-19T23:30:35AEST	77	0 2023-04-19T23:31:23AEST	80	0 2023-04-19T23:32:58AEST	81	0 2023-04-19T23:33:18AEST	83	6 2023-04-19T23:34:10AEST	84	0 2023-04-19T23:45:06AEST	86	0 2023-04-19T23:45:13AEST	87	0" | tr ' ' '\n'`
	assert_result

	echo "$func_name, DONE" > /dev/stderr
}
#	}}}

test_parse() {
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
	local test_num=1
	echo "$func_name, UNIMPLEMENTED" > /dev/stderr; exit 2;
	echo "$func_name, DONE" > /dev/stderr
}

test_filter() {
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
	local test_num=1
	echo "$func_name, warning, UNIMPLEMENTED" > /dev/stderr; exit 2;
	echo "$func_name, DONE" > /dev/stderr
}

test_count() {
#	{{{
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
	local test_num=1

	test_cmd=( $cmd_datetimescan count --per "y" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023: 5"
	assert_result

	test_cmd=( $cmd_datetimescan count --per "m" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05: 5"
	assert_result

	test_cmd=( $cmd_datetimescan count --per "d" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05-05: 5"
	assert_result

	test_cmd=( $cmd_datetimescan count --per "y" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023: 55
2024: 1"
	assert_result

	test_cmd=( $cmd_datetimescan count --per "m" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-04: 55
2024-04: 1"
	assert_result

	test_cmd=( $cmd_datetimescan count --per "d" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-04-19: 55
2024-04-19: 1"
	assert_result

	echo "$func_name, DONE" > /dev/stderr
}
#	}}}

test_deltas() {
#	{{{
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
	local test_num=1

	test_cmd=( $cmd_datetimescan deltas --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $cmd_datetimescan deltas --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"41
21
34
17"
	assert_result

	test_cmd=( $cmd_datetimescan deltas --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "36 170 31622508 0 5 71 27 30 23 88 82 14 72 0 189 193 17 71 85 23 25 27 38 95 8 14 41 49 23 39 97 48 7 26 21 31 28 5 26 2466 10 15 34 28 348 27 15 15 46 48 95 20 52 656 7" | tr ' ' '\n'`
	assert_result

	test_cmd=( $cmd_datetimescan deltas --allow_negative --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "36 170 31622508 -31622319 5 71 27 30 23 88 82 14 72 -62 189 193 17 71 85 23 25 27 38 95 8 14 41 49 23 39 97 48 7 26 21 31 28 5 26 2466 10 15 34 28 348 27 15 15 46 48 95 20 52 656 7" | tr ' ' '\n'`
	assert_result

	echo "$func_name, DONE" > /dev/stderr
}
#	}}}

test_splits() {
#	{{{
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
	local test_num=1

	test_cmd=( $cmd_datetimescan splits --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $cmd_datetimescan splits --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"113"
	assert_result

	test_cmd=( $cmd_datetimescan splits --input "$path_testfile_isodatetimes" --timeout 1 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "" | tr ' ' '\n'`
	assert_result

	test_cmd=( $cmd_datetimescan splits --input "$path_testfile_isodatetimes" --timeout 30 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "21 17" | tr ' ' '\n'`
	assert_result

	test_cmd=( $cmd_datetimescan splits --input "$path_testfile_isodatetimes" --timeout 1200 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "113" | tr ' ' '\n'`
	assert_result

	test_cmd=( $cmd_datetimescan splits --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "206 1638 87 318 7" | tr ' ' '\n'`
	assert_result

	test_cmd=( $cmd_datetimescan splits --input "$path_testfile_isodatetimes_2" --timeout 1 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "" | tr ' ' '\n'`
	assert_result

	test_cmd=( $cmd_datetimescan splits --input "$path_testfile_isodatetimes_2" --timeout 600 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "206 1638 753 7" | tr ' ' '\n'`
	assert_result

	test_cmd=( $cmd_datetimescan splits --input "$path_testfile_isodatetimes_2" --timeout 1200 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "206 1638 1416" | tr ' ' '\n'`
	assert_result

	echo "$func_name, DONE" > /dev/stderr
}
#	}}}

test_sum() {
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
	local test_num=1

	test_cmd=( $cmd_datetimescan sum --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $cmd_datetimescan sum --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05-05: 113"
	assert_result

	test_cmd=( $cmd_datetimescan sum --per "d" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05-05: 113"
	assert_result

	test_cmd=( $cmd_datetimescan sum --per "m" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05: 113"
	assert_result

	test_cmd=( $cmd_datetimescan sum --per "y" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023: 113"
	assert_result

	test_cmd=( $cmd_datetimescan sum --per "d" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-04-19: 2445"
	assert_result

	echo "$func_name, INCOMPLETE" > /dev/stderr
	echo "$func_name, DONE" > /dev/stderr
}

test_wpm() {
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
	local test_num=1
	echo "$func_name, UNIMPLEMENTED" > /dev/stderr; exit 2;
	echo "$func_name, DONE" > /dev/stderr
}

assert_result() {
	if [[ $flag_print_results -ne 0 ]]; then
		print_result
	fi
	if [[ ! "$result_str" == "$expected_str" ]]; then
		echo "$func_name, fail: $test_num\n" > /dev/stderr
		#	use 'if' to prevent errexit triggering (diff returns rc=1)
		if diff --color -u <( echo $result_str ) <( echo $expected_str ) > /dev/stderr; then echo "" > /dev/null; fi
		#	{{{
		#if diff --color <( echo $result_str ) <( echo $expected_str ); then echo "" > /dev/null; fi
		#if diff <( echo $result_str ) <( echo $expected_str ); then echo "" > /dev/null; fi
		#diff --color --suppress-common-lines -y <( echo $result_str ) <( echo $expected_str )
		#	}}}
		failures_count=`perl -E "say $failures_count + 1"`
		if [[ $flag_exit_on_fail -ne 0 ]]; then
			exit 2
		fi
	fi
	test_num=$( perl -E "say $test_num + 1" )
}

print_result() {
	num=60
	prefix_str="=== $func_name: $test_num, result: "
	num_equals=$(($num - ${#prefix_str}))
	if ((num_equals < 1)); then num_equals=1; fi  
	output_str="${prefix_str}$(printf '=%.0s' $(seq 1 $num_equals))"
	echo "$output_str" > /dev/stderr
	echo "$result_str" > /dev/stderr
	printf '=%.0s' $(seq 1 $num) > /dev/stderr
	echo "" > /dev/stderr
}

main "$@"


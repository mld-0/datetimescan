#!/usr/bin/env zsh
#	vim-modelines:	{{{3
#   vim: set tabstop=4 modeline modelines=10:
#   vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
#	{{{2

#	Notes:
#	{{{
#	2023-05-20T21:10:59AEST instead of calling this from Rust - just write it in Rust?
#	2023-05-20T21:12:04AEST PROJECT_PATH is correct even when running script with `:w !zsh`?
#	2023-05-23T20:58:38AEST verify intervals are reported in sorted order
#	}}}

set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

#set -x 

microtime() { python3 -c 'import time; print(int(time.time_ns()/1000000))' }
echoerr() { echo "$@" | sed "s|$HOME|~|g" > /dev/stderr; }

SCRIPTPATH="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
PROJECT_PATH=$( dirname "$SCRIPTPATH" )
#	validate: SCRIPTPATH, PROJECT_PATH
#	{{{
if [[ ! -d "$SCRIPTPATH" ]]; then
	echoerr "Failed to find dir SCRIPTPATH=($SCRIPTPATH)"
	exit 2
fi
if [[ ! -d "$PROJECT_PATH" ]]; then
	echoerr "Failed to find PROJECT_PATH=($PROJECT_PATH)"
	exit 2
fi
#	}}}

bin_cargo="$HOME/.cargo/bin/cargo"
path_testfile_isodatetimes="$PROJECT_PATH/tests/data/textWithIsoDatetimes-1.txt"
path_testfile_isodatetimes_2="$PROJECT_PATH/tests/data/textWithIsoDatetimes-2.txt"
path_testfile_worklog_samples="$PROJECT_PATH/tests/data/worklog.scrambled.samples.txt"
path_testfile_empty="$PROJECT_PATH/tests/data/empty.txt"
#	validate: bin_cargo, path_testfile_*
#	{{{
if [[ ! -x "$bin_cargo" ]]; then
	echoerr "Failed to find bin_cargo=($bin_cargo)"
	exit 2
fi
if [[ ! -f "$path_testfile_isodatetimes" ]]; then
	echoerr "Failed to find path_testfile_isodatetimes=($path_testfile_isodatetimes)"
	exit 2
fi
if [[ ! -f "$path_testfile_isodatetimes_2" ]]; then
	echoerr "Failed to find path_testfile_isodatetimes_2=($path_testfile_isodatetimes_2)"
	exit 2
fi
if [[ ! -f "$path_testfile_empty" ]]; then
	echoerr "Failed to find path_testfile_empty=($path_testfile_empty)"
	exit 2
fi
if [[ ! -f "$path_testfile_worklog_samples" ]]; then
	echoerr "Failed to find path_testfile_worklog_samples=($path_testfile_worklog_samples)"
	exit 2
fi
#	}}}

cmd_build=( $bin_cargo build --release  )
bin_datetimescan="$PROJECT_PATH/target/release/datetimescan"

flag_print_output=0
flag_report_pass=1
flag_exit_on_check_comparison_fail=0
rust_log_level=''		#	set to 'trace' for full output

main() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	if [[ ! -z "$rust_log_level" ]]; then
		export RUST_LOG="$rust_log_level";
	fi
	local failures_count=0
	build_release
	run_tests
	report_failures
}

run_tests() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local startTime=$( microtime )
	test_scan
	#test_parse
	#test_convert
	#test_filter
	test_count
	test_deltas
	test_splits
	test_sum
	#test_wpm
	local endTime=$( microtime )
	local elapsedTime=$( perl -E "say $endTime - $startTime" )
	echoerr "$func_name, DONE"
	echoerr "$func_name, runtime: $elapsedTime ms"
}


test_scan() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local result_str=""
	local expected_str=""
	local test_num=1

	test_cmd=( $bin_datetimescan scan )
	result_str=$( echo "" | ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $bin_datetimescan scan --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $bin_datetimescan scan --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05-05T19:34:42+1000	1	0
2023-05-05T19:35:23+1000	2	0
2023-05-05T19:35:44+1000	3	10
2023-05-05T19:36:18+1000	4	0
2023-05-05T19:36:35+1000	5	0"
	assert_result

	test_cmd=( $bin_datetimescan scan --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "2023-04-19T22:07:40AEST	1	10 2023-04-19T22:08:16AEST	2	0 2023-04-19T22:11:06AEST	4	10 2024-04-19T22:12:54AEST	6	0 2023-04-19T22:14:15AEST	10	0 2023-04-19T22:14:20AEST	12	6 2023-04-19T22:15:31AEST	13	0 2023-04-19T22:15:58AEST	15	9 2023-04-19T22:16:28AEST	16	0 2023-04-19T22:16:51AEST	18	0 2023-04-19T22:18:19AEST	19	0 2023-04-19T22:19:41AEST	21	6 2023-04-19T22:19:55AEST	22	0 2023-04-19T22:21:07AEST	24	9 2023-04-19T22:20:05AEST	25	6 2023-04-19T22:23:14AEST	26	0 2023-04-19T22:26:27AEST	28	0 2023-04-19T22:26:44AEST	29	0 2023-04-19T22:27:55AEST	31	0 2023-04-19T22:29:20AEST	32	0 2023-04-19T22:29:43AEST	34	9 2023-04-19T22:30:08AEST	35	0 2023-04-19T22:30:35AEST	37	0 2023-04-19T22:31:13AEST	38	0 2023-04-19T22:32:48AEST	39	0 2023-04-19T22:32:56AEST	41	0 2023-04-19T22:33:10AEST	42	10 2023-04-19T22:33:51AEST	43	6 2023-04-19T22:34:40AEST	44	0 2023-04-19T22:35:03AEST	46	6 2023-04-19T22:35:42AEST	47	0 2023-04-19T22:37:19AEST	49	9 2023-04-19T22:38:07AEST	50	0 2023-04-19T22:38:14AEST	52	0 2023-04-19T22:38:40AEST	53	0 2023-04-19T22:39:01AEST	55	0 2023-04-19T22:39:32AEST	56	0 2023-04-19T22:40:00AEST	59	0 2023-04-19T22:40:05AEST	60	0 2023-04-19T22:40:31AEST	61	0 2023-04-19T23:21:37AEST	63	0 2023-04-19T23:21:47AEST	64	0 2023-04-19T23:22:02AEST	67	0 2023-04-19T23:22:36AEST	68	0 2023-04-19T23:23:04AEST	69	0 2023-04-19T23:28:52AEST	71	10 2023-04-19T23:29:19AEST	73	7 2023-04-19T23:29:34AEST	74	0 2023-04-19T23:29:49AEST	76	6 2023-04-19T23:30:35AEST	77	0 2023-04-19T23:31:23AEST	80	0 2023-04-19T23:32:58AEST	81	0 2023-04-19T23:33:18AEST	83	6 2023-04-19T23:34:10AEST	84	0 2023-04-19T23:45:06AEST	86	0 2023-04-19T23:45:13AEST	87	0" | tr ' ' '\n'`
	assert_result

	echoerr "$func_name, DONE"
}


test_parse() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local result_str=""
	local expected_str=""
	local test_num=1
	echoerr "$func_name, UNIMPLEMENTED"; exit 2;
	echoerr "$func_name, DONE"
}


test_convert() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local result_str=""
	local expected_str=""
	local test_num=1
	echoerr "$func_name, WARNING, UNIMPLEMENTED"; exit 2;
	echoerr "$func_name, DONE"
}


test_filter() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local result_str=""
	local expected_str=""
	local test_num=1
	echoerr "$func_name, WARNING, UNIMPLEMENTED"; exit 2;
	echoerr "$func_name, DONE"
}


test_count() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local result_str=""
	local expected_str=""
	local test_num=1

	test_cmd=( $bin_datetimescan count --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $bin_datetimescan count --per "all" --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $bin_datetimescan count --per "y" --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $bin_datetimescan count --per "m" --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $bin_datetimescan count --per "d" --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $bin_datetimescan count --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"5"
	assert_result

	test_cmd=( $bin_datetimescan count --per "all" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"5"
	assert_result

	test_cmd=( $bin_datetimescan count --per "y" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023: 5"
	assert_result

	test_cmd=( $bin_datetimescan count --per "m" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05: 5"
	assert_result

	test_cmd=( $bin_datetimescan count --per "d" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05-05: 5"
	assert_result

	test_cmd=( $bin_datetimescan count --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"56"
	assert_result

	test_cmd=( $bin_datetimescan count --per "all" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"56"
	assert_result

	test_cmd=( $bin_datetimescan count --per "y" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023: 55
2024: 1"
	assert_result

	test_cmd=( $bin_datetimescan count --per "m" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-04: 55
2024-04: 1"
	assert_result

	test_cmd=( $bin_datetimescan count --per "d" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-04-19: 55
2024-04-19: 1"
	assert_result

	test_cmd=( $bin_datetimescan count --per "all" --input "$path_testfile_worklog_samples" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"346"
	assert_result

	test_cmd=( $bin_datetimescan count --per "y" --input "$path_testfile_worklog_samples" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2022: 62
2023: 284"
	assert_result

	test_cmd=( $bin_datetimescan count --per "m" --input "$path_testfile_worklog_samples" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2022-06: 6
2022-07: 15
2022-09: 9
2022-11: 13
2022-12: 19
2023-01: 5
2023-02: 15
2023-03: 12
2023-04: 64
2023-05: 188"
	assert_result

	test_cmd=( $bin_datetimescan count --per "d" --input "$path_testfile_worklog_samples" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2022-06-05: 5
2022-06-06: 1
2022-07-13: 13
2022-07-14: 2
2022-09-03: 9
2022-11-05: 13
2022-12-11: 8
2022-12-27: 11
2023-01-01: 5
2023-02-20: 15
2023-03-29: 12
2023-04-13: 64
2023-05-01: 20
2023-05-02: 9
2023-05-03: 11
2023-05-04: 4
2023-05-05: 17
2023-05-06: 12
2023-05-08: 9
2023-05-09: 6
2023-05-11: 5
2023-05-14: 29
2023-05-15: 13
2023-05-18: 4
2023-05-19: 9
2023-05-20: 22
2023-05-22: 12
2023-05-23: 6"
	assert_result

	echoerr "$func_name, DONE"
}


test_deltas() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local result_str=""
	local expected_str=""
	local test_num=1

	test_cmd=( $bin_datetimescan deltas --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $bin_datetimescan deltas --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"41
21
34
17"
	assert_result

	test_cmd=( $bin_datetimescan deltas --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "36 170 31622508 0 5 71 27 30 23 88 82 14 72 0 189 193 17 71 85 23 25 27 38 95 8 14 41 49 23 39 97 48 7 26 21 31 28 5 26 2466 10 15 34 28 348 27 15 15 46 48 95 20 52 656 7" | tr ' ' '\n'`
	assert_result

	test_cmd=( $bin_datetimescan deltas --allow_negative --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "36 170 31622508 -31622319 5 71 27 30 23 88 82 14 72 -62 189 193 17 71 85 23 25 27 38 95 8 14 41 49 23 39 97 48 7 26 21 31 28 5 26 2466 10 15 34 28 348 27 15 15 46 48 95 20 52 656 7" | tr ' ' '\n'`
	assert_result

	echoerr "$func_name, DONE"
}


test_splits() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local result_str=""
	local expected_str=""
	local test_num=1

	test_cmd=( $bin_datetimescan splits --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $bin_datetimescan splits --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"113"
	assert_result

	test_cmd=( $bin_datetimescan splits --per "all" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"113"
	assert_result

	test_cmd=( $bin_datetimescan splits --per "y" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023: 113"
	assert_result

	test_cmd=( $bin_datetimescan splits --per "m" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05: 113"
	assert_result

	test_cmd=( $bin_datetimescan splits --per "d" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05-05: 113"
	assert_result

	test_cmd=( $bin_datetimescan splits --input "$path_testfile_isodatetimes" --timeout 1 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "" | tr ' ' '\n'`
	assert_result

	test_cmd=( $bin_datetimescan splits --input "$path_testfile_isodatetimes" --timeout 30 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "21 17" | tr ' ' '\n'`
	assert_result

	test_cmd=( $bin_datetimescan splits --input "$path_testfile_isodatetimes" --timeout 1200 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "113" | tr ' ' '\n'`
	assert_result

	test_cmd=( $bin_datetimescan splits --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "206 1638 87 318 7" | tr ' ' '\n'`
	assert_result

	test_cmd=( $bin_datetimescan splits --input "$path_testfile_isodatetimes_2" --timeout 1 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "" | tr ' ' '\n'`
	assert_result

	test_cmd=( $bin_datetimescan splits --input "$path_testfile_isodatetimes_2" --timeout 600 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "206 1638 753 7" | tr ' ' '\n'`
	assert_result

	test_cmd=( $bin_datetimescan splits --input "$path_testfile_isodatetimes_2" --timeout 1200 )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
`echo "206 1638 1416" | tr ' ' '\n'`
	assert_result

	test_cmd=( $bin_datetimescan splits --per "y" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023: 2033, 87, 318, 7"
	assert_result

	test_cmd=( $bin_datetimescan splits --per "m" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-04: 2033, 87, 318, 7"
	assert_result

	test_cmd=( $bin_datetimescan splits --per "d" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-04-19: 2033, 87, 318, 7"
	assert_result

	echoerr "$func_name, DONE"
}


test_sum() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local result_str=""
	local expected_str=""
	local test_num=1

	test_cmd=( $bin_datetimescan sum --input "$path_testfile_empty" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
""
	assert_result

	test_cmd=( $bin_datetimescan sum --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"113"
	assert_result

	test_cmd=( $bin_datetimescan sum --per "all" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"113"
	assert_result

	test_cmd=( $bin_datetimescan sum --per "y" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023: 113"
	assert_result

	test_cmd=( $bin_datetimescan sum --per "m" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05: 113"
	assert_result

	test_cmd=( $bin_datetimescan sum --per "d" --input "$path_testfile_isodatetimes" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-05-05: 113"
	assert_result

	test_cmd=( $bin_datetimescan sum --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2256"
	assert_result

	test_cmd=( $bin_datetimescan sum --per "all" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2256"
	assert_result

	test_cmd=( $bin_datetimescan sum --per "y" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023: 2445"
	assert_result

	test_cmd=( $bin_datetimescan sum --per "m" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-04: 2445"
	assert_result

	test_cmd=( $bin_datetimescan sum --per "d" --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"2023-04-19: 2445"
	assert_result

	test_cmd=( $bin_datetimescan sum --timeout 1200 --input "$path_testfile_isodatetimes_2" )
	result_str=$( ${test_cmd[@]} )
	expected_str=\
"3260"
	assert_result

	echoerr "$func_name, DONE"
}


test_wpm() {
	#	funcname: {{{
	local func_name=""
	if [[ -n "${ZSH_VERSION:-}" ]]; then 
		func_name=${funcstack[1]:-}
	elif [[ -n "${BASH_VERSION:-}" ]]; then
		func_name="${FUNCNAME[0]:-}"
	else
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	local result_str=""
	local expected_str=""
	local test_num=1
	echoerr "$func_name, UNIMPLEMENTED"; exit 2;
	echoerr "$func_name, DONE"
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
		printf "%s\n" "WARNING, func_name unset, non zsh/bash shell" > /dev/stderr
	fi
	#	}}}
	echoerr "PROJECT_PATH=($PROJECT_PATH)"
	echoerr "cmd_build=(${cmd_build[@]})"
	cd "$PROJECT_PATH"
	${cmd_build[@]}
	if [[ ! -x "$bin_datetimescan" ]]; then
		echoerr "$func_name, failed to bin_datetimescan=($bin_datetimescan)"
		exit 2
	fi
	echoerr "$func_name, DONE"
}
#	}}}

assert_result() {
#	{{{
	if [[ $flag_print_output -ne 0 ]]; then
		print_result
	fi
	if [[ ! "$result_str" == "$expected_str" ]]; then
		echoerr "$func_name, fail: $test_num\n"
		#	use 'if' to prevent errexit triggering (diff returns rc=1)
		if diff --color -u <( echo $result_str ) <( echo $expected_str ) > /dev/stderr; then echo "" > /dev/null; fi
		#	{{{
		#if diff --color <( echo $result_str ) <( echo $expected_str ); then echo "" > /dev/null; fi
		#if diff <( echo $result_str ) <( echo $expected_str ); then echo "" > /dev/null; fi
		#diff --color --suppress-common-lines -y <( echo $result_str ) <( echo $expected_str )
		#	}}}
		failures_count=`perl -E "say $failures_count + 1"`
		if [[ $flag_exit_on_check_comparison_fail -ne 0 ]]; then
			exit 2
		fi
	else
		if [[ $flag_report_pass -ne 0 ]]; then
			echoerr "$func_name, pass: $test_num"
		fi
	fi
	test_num=$( perl -E "say $test_num + 1" )
}
#	}}}

print_result() {
#	{{{
	num=60
	prefix_str="=== $func_name: $test_num, result: "
	num_equals=$(($num - ${#prefix_str}))
	if ((num_equals < 1)); then num_equals=1; fi  
	output_str="${prefix_str}$(printf '=%.0s' $(seq 1 $num_equals))"
	echoerr "$output_str"
	echoerr "$result_str"
	printf '=%.0s' $(seq 1 $num) > /dev/stderr
	echoerr ""
}
#	}}}

report_failures() {
#	{{{
	if [[ $failures_count -gt 0 ]]; then
		echoerr "$func_name, failures_count=($failures_count)"
		exit 2
	fi
}
#	}}}

main "$@"


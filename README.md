# datetimescan

## About:

Command-line datetimes-in-file analysis utility 

## Usage:

    datetimescan [common args] [command] [command args]

### Common arguments

        --input             [file] input (default=stdin)
        --output            UNIMPLEMENTED [file] output (default=stdout)
        --filter_start      [date] Exclude dates before
        --filter_end        [date] Exclude dates after
        --filter_invert     Invert filter excluded items
        --no_future         Error for datetimes in the future
        --no_unsorted       Error for datetimes not in order
        --custom_format     UNIMPLEMENTED [format] Add custom datetime format
        --only_format       UNIMPLEMENTED [format] Only use given datetime format
        --exclude_format    UNIMPLEMENTED [format] Do not use given datetime format
        --assumetz          UNIMPLEMENTED [tz] Timezone to assume where not given (default=system)

### Commands

#### locate

List datetime matches and their locations
Datetimes are not parsed (therefore arguments like --no_future, --no_unsorted, and --filter_* have no effect)

        --no_locations       Do not report locations

#### parse

UNIMPLEMENTED List datetime matches in specified output format

        --outputfmt     UNIMPLEMENTED [format] Custom output datetime format

#### convert

UNIMPLEMENTED Print input, with datetimes converted to specified output format

        --outputfmt     UNIMPLEMENTED [format] Custom output datetime format

#### filter

UNIMPLEMENTED Print input, excluding lines containing datetimes outside filter range

#### count

Count datetimes per interval

        --per       [interval] (y/m/d/all)

#### deltas

Report seconds elapsed between each datetime match

        --allow_negative    Include negative deltas

#### splits

Report length of continuous deltas where no delta > timeout

        --timeout       [delta] Max delta to consider continuous (default=300)
        --per           [interval] (y/m/d/all)
        --unit          [unit] output in seconds/minutes/hours (s/m/h) (default=s)
        --breakempty    UNIMPLEMENTED End continuious deltas if there is an empty line between them

#### sum

Sum splits per interval

        --timeout       [delta] Max delta to consider continuous (default=300)
        --per           [interval] (y/m/d/all)
        --unit          [unit] output in seconds/minutes/hours (s/m/h) (default=s)
        --breakempty    UNIMPLEMENTED End continuious deltas if there is an empty line between them

#### groupsum

UNIMPLEMENTED ...

        --groups        [GROUPS] file containing list of repos to group by

#### wpms

UNIMPLEMENTED ...

## Supported datetime formats:

    2023-05-08T19:29:50AEST 
    2023-05-08T19:29:50UTC 
    2023-05-08T19:29:50+1000
    2023-05-08T19:29:50+10:00
    2023-05-08T19:29:50
    2023-05-08 19:29:50

Regex is: `r"(?P<datetime>\d{4}-\d{2}-\d{2}[ T]\d{2}:\d{2}:\d{2}(?:[A-Z]{3,4}|[+-]\d{2}:?\d{2})?)"`

Note: only (currently) parsable letter-code timezones are: UTC, AEST, AEDT

## Notes:

Replaces (disaster-made-of-python) 'dtscan' (see <>)

So far largely a case-study in asking gpt4 to write functions and their docs (but not in how the whole thing might be laid out - see Continues on how datetimescan::*_datetimes::* always should be been combined).

For unsorted datetime input, totals from 'splits' / 'sum' may differ between --per 'all' / 'ymd'

Not all commands/arguments are implemented.



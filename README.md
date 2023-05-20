# datetimescan

## About:

Command-line datetimes-in-file analysis utility 

## Usage:

Common arguments

        --input             [file] input (default=stdin)
        --filter_start      [date] Exclude dates before
        --filter_end        [date] Exclude dates after
        --filter_invert     Invert filter excluded items
        --custom_format     [format] Custom datetime format
        --assumetz          [tz] Timezone to assume where not given (default=system)

Commands

### scan

List datetime matches and their locations

        --nolocations       Do not report locations

## parse

List datetime matches in specified output format

        --outputfmt     [format] Custom output datetime format

### filter

Print input, excluding lines containing datetimes outside filter range

### count

Count datetimes per interval

        --per       [interval] (y/m/d)

### deltas

Report seconds elapsed between each datetime match

        --onlypositive      Do not include negative deltas

### splits

Report length of continuous deltas where no delta > timeout

        --timeout       [delta] Max delta to consider continuous (default=300)
        --breakempty    End continuious deltas if there is an empty line between them

### sum

Sum splits per interval

        --timeout       [delta] Max delta to consider continuous (default=300)
        --per           [interval] (y/m/d)
        --breakempty    End continuious deltas if there is an empty line between them

### wpms

...

## Supported datetime formats:

    2023-05-08T19:29:50AEST 
    2023-05-08T19:29:50UTC 
    2023-05-08T19:29:50+1000
    2023-05-08T19:29:50+10:00
    2023-05-08 19:29:50
    2023-05-08T19:29:50

Only (currently) valid letter-code timezones are: UTC, AEST, AEDT

## Notes:

Replaces (disaster-made-of-python) 'dtscan' (see <>)

So far largely a case-study in asking gpt4 to write functions and their docs (but not in how the whole thing might be laid out - see Continues on how datetimescan::*_datetimes::* always should be been combined).


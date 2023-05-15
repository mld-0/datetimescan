# datetimescan

Command-line datetimes-in-file analysis utility

## Common arguments:

        --input             [file] input (default=stdin)
        --filter_start      [date] Exclude dates before
        --filter_end        [date] Exclude dates after
        --custom_format     [format] Custom datetime format

## Commands:

### scan

Report datetime matches and their locations

        --nolocations       Do not report locations


### count

Count datetimes per interval

        --per       [interval] (y/m/d)

### deltas

Report seconds elapsed between each datetime match

        --onlypositive      Do not include negative deltas

### splits

Report length of continuous deltas where no delta > timeout

        --timeout       [delta] Max delta to consider continuous (default=300)

### sum

Sum splits per interval

        --timeout       [delta] Max delta to consider continuous (default=300)
        --per           [interval] (y/m/d)

### wpms

...





Commands:

scan
Report datetime matches and their locations
        --nolocations       Do not report locations


count
Count datetimes per interval
        --per               [y/m/d]

deltas
Report seconds elapsed between each datetime match
        --onlypositive      Do not include negative deltas

splits
Report length of continuous intervals where no delta is > timeout
        --timeout           [default=300] Max delta to consider continuous

sum
Sum splits per interval
        --timeout           [default=300] Max delta to consider continuous
        --per               [y/m/d]

wpms

Common Options:
        --input             [default=stdin] input file
        --filter_start
        --filter_end
        --custom_format


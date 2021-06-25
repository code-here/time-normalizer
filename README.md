# time normalizer

this tool converts a single time unit value to readable time.

## usage

clone this repository and change directory:

    git clone https://github.com/code-here/time-normalizer
    cd time-normalizer

run it like this:

    cargo run -- <time to convert>

examples:

    cargo run -q -- 12000s  // 3 hours 20 minutes
    cargo run -q -- 549m // 9 hours 9 minutes
    cargo run -q -- 388min // 6 hours 28 minutes

acronyms for:

year: y, yr, years
day: d, dy, days
hour: h, hr, hours
minute: m, min, minutes
second: s, sec, seconds

### note

An year is considered as 365 days in calculation

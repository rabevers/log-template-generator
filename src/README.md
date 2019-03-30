# Log template generator

Command line tool to generate templates for the logs I keep. It is my first project using Rust.

It generates a directory for each week. The directory will have two Markdown files. One for the things I did and another for the hours I worked.


so far it generates the log files, but only for week 11 in 2019.

# Todo
- Make the year configurable
- Make the week number configurable
- If no year is provided, the current year should be used
- If a week nuber is provided that already has a directory, the files in that directory are overwritten. This should not be possible and should generate an error.
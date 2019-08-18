#!/usr/bin/fish --

set single_dir "../../../results_single"
set arg2 (echo $argv) 

for x in (ls "$single_dir")
    eval "$arg2" $single_dir/$x
end
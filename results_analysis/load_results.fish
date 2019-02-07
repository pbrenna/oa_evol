function load_results
    set list 0 0 0
    for x in *.log
        set parts (string split " " (tail -n 3 $x))
        set number (string sub -s 1 -l 3 $x)
        if test "$parts[5]" = "suitable"
            if test "$parts[12]" = "NaN%"
                set parts[12] ""
            end
            set list[$number] "$parts[4], $parts[12], $parts[8]"
        else if test "$parts[3]" = "Not"
            set failed_runs (cat $x | grep "Not" | wc -l)
            set successful_runs (cat $x | grep "Found" | wc -l)
            set total_runs (math $failed_runs + $successful_runs)
            set list[$number] "$successful_runs, , $total_runs"
        end
    end
end
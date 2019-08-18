#!/usr/bin/fish --
set logdir "../../../results"
set outdir "../../../results_single"
set contatore 0
for x in (find "$logdir" -iname "*.log")
    cat $x | awk '/Found OA/{ print "" } /^[0 | 1]+$/{ printf "%sn", $0 }' | while read oa
        echo -n $oa | tr "n" "\n" > "$outdir/$contatore.oa.txt"
        set contatore (math $contatore + 1)
    end
end
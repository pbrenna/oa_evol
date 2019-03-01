import csv

all = ""
makefile = ""
fitness_map = {
    'delta': 'DeltaFast',
    'walsh': 'WalshFast',
    'cidev': 'Cidev'
}
with open("tests.csv") as f:
    csv_reader = csv.reader(f, delimiter='\t')
    line = 0
    for row in csv_reader:
        line += 1
        if row[0] == "":
            continue
        algo = row[0]
        part1 = ""
        folder = "ga_algo"
        fitness = row[1]
        exponent = row[2]
        depth = row[3]
        parts = row[4].split(",")
        parts = [p.strip() for p in parts] 
        N = parts[0]
        k = parts[1]
        t = parts[2]
        outfile = "{:03d}.{}.{}.{}.{}.{}".format(line, algo, fitness, N, k, t)
        if exponent != "":
            outfile += ".exp{}".format(exponent)
        if depth != "":
            outfile += ".depth{}".format(depth)
        outfile += ".log"
        if exponent == "":
            exponent = 2
        if algo in ["gp", "gp_inc"]:
            folder = "gp_algo"
            part1 = "--max-depth {}".format(depth) if depth != "" else ""
        if algo == "ga_inc":
            folder = "ga_inc"
        if algo == "gp_inc":
            folder = "gp_inc"
        makefile += """\n$(outdir)/{}:
\t@mkdir -p $(outdir)
\tcargo run --bin {} --release {} {} {} --fitness {} --fitness-exp {} {} --runs $(runs) --log $@ --threads $(threads)
\t@git --no-pager log -1 --pretty=format:"%nCommit: %h %d%n" >> $@
""".format(outfile, folder, N, k, t, fitness_map[fitness], exponent, part1, outfile, outfile, outfile)
        all += " $(outdir)/{}".format(outfile)
makefile = """threads = 2
runs = 50
outdir = results

all: {} 
""".format(all) + makefile
print(makefile)

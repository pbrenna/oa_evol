import csv

all = ""
runs = 100
makefile = ""
fitness_map = {
    'delta': 'DeltaFast',
    'walsh': 'WalshFast',
    'delta_bal' : 'DeltaFast',
    'walsh_bal' : 'WalshFast'
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
        outfile = "{:03d}.{}.{}.{}.{}".format(line, algo, N, k, t)
        if exponent != "":
            outfile += ".exp{}".format(exponent)
        if depth != "":
            outfile += ".depth{}".format(depth)
        outfile += ".{}runs.log".format(runs)
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
\tmkdir -p $(outdir) && cd {} && cargo run --release {} {} {} --fitness {} --fitness-exp {} {} --runs {} --log ../$(outdir)/{} --threads $(threads)
""".format(outfile, folder, N, k, t, fitness_map[fitness], exponent, part1, runs, outfile)
        all += " $(outdir)/{}".format(outfile)
makefile = """threads = 2

outdir = results/

all: {} 
""".format(all) + makefile
print(makefile)

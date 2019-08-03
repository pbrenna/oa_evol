import sys
import itertools

columns = []
for line in sys.stdin:
    line = line.strip()
    if len(line) > 0:
        row = line.split(" ")
        row = [True if x == '1' else False for x in row]
        while len(columns) < len(row):
            columns.append([])
        for i, val in enumerate(row):
            columns[i].append(val)

try:
    strength = int(sys.argv[1])
except:
    print("Fornire parametro t")
    sys.exit()
print("Desired strength: {}".format(strength))

try:
    for w in range(1, strength+1):
        for i in itertools.combinations(columns, w):
            submatrix_rows = zip(*i)
            counts = {}
            for row in submatrix_rows:
                h = row
                if h not in counts:
                    counts[h] = 0
                counts[h] += 1
            for key in counts:
                first = counts[key]
                break
            for key in counts:
                assert counts[key] == first
    print("È ortogonale: OA({}, {}, 2, {})".format(len(columns[0]),
                                                   len(columns),
                                                   strength))
except AssertionError:
    print("NON è ortogonale")

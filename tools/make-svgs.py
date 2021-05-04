import subprocess
import sys

fileNum = int(sys.argv[1])

for i in range(1, fileNum+1):
    res = subprocess.call(["cargo", "run", "--release", "--bin", "vis", "in/0000.txt", f"output/{i}.txt", f"{i}"])
    if res != 0:
        print(f"converting {i}.txt is error", file=sys.stderr)
        exit(1)

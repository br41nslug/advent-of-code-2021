# python solution for day 1
import sys

inp = [int(line.strip()) for line in sys.stdin.readlines()]
counter = 0
last = inp.pop(0)
for item in inp:
    if item > last:
        counter = counter + 1
    last = item
print(counter)

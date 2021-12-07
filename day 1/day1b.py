# python solution for day 1 part 2
import sys

inp = [int(line.strip()) for line in sys.stdin.readlines()]
counter = 0
window = [inp.pop(0), inp.pop(0), inp.pop(0)]
last = sum(window)
for item in inp:
    window.pop(0)
    window.append(item)
    total = sum(window)
    if total > last:
        counter = counter + 1
    last = total
print(counter)

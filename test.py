f = open("/Users/jakelanders/code/advent-of-code-2022/src/lib/day1.txt", "r")

m = [0, 0, 0]

f = f.read()

for line in f.split("\n"):
    if line != "\n":
        items = line.split("\n")

        s = 0

        for i in items:
            fx = i.replace("\n", "")
            if fx != "":
                s += int(fx)

        for i in range(0, 3):
            if s > m[i]:
                m[i] = s
                break

print(sum(m))

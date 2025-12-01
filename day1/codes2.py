with open("codes.txt") as f:
    lines = f.read().split("\n")

    dial = 50
    zeroes = 0
    for line in lines:
        if line == '': continue
        print(f"{dial} | {line} | {zeroes}")

        magnitude = int(line[1:])
        direction = -1 if line[0] == 'L' else 1

        while (magnitude > 0):
            dial = (dial + direction) % 100
            magnitude -= 1
            if dial == 0: zeroes += 1

    print(f"{zeroes=}")

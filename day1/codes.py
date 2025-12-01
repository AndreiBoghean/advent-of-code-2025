with open("codes.txt") as f:
    lines = f.read().split("\n")

    dial = 50
    zeroes = 0
    boundaryBreaks = 0
    for line in lines:
        print(f"{dial} | {line} | {zeroes}")
        if line == '': continue

        magnitude = int(line[1:])
        loops = magnitude // 100
        direction = -1 if line[0] == 'L' else 1
        delta = dial if line[0] == 'L' and dial != 0 else 100-dial


        # exhaust loops of movement
        if loops >= 1:
            boundaryBreaks += loops
            magnitude -= 100 * loops

        # move to the next position
        if magnitude > delta: boundaryBreaks += 1
        elif dial == 0: zeroes += 1

        dial += magnitude * direction
        dial %= 100

    print(f"{zeroes+boundaryBreaks=}")

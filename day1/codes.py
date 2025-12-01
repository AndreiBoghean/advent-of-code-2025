with open("/tmp/codes.txt") as f:
    lines = f.read().split("\n")

    dial = 50
    zeroes = 0
    boundaryBreaks = 0
    for line in lines:
        # print(f"{dial} | {line} | {zeroes} | {boundaryBreaks}")
        if line == '': continue
        print(f"{dial} | {line} | {zeroes+boundaryBreaks}")

        magnitude = int(line[1:])
        loops = magnitude // 100
        direction = -1 if line[0] == 'L' else 1
        delta = dial if line[0] == 'L' and dial != 0 else 100-dial


        # exhaust loops of movement
        if loops >= 1:
            print("bb loop")
            boundaryBreaks += loops
            magnitude -= 100 * loops
            # if dial == 0:
            #     boundaryBreaks -= 1
            #     print("bb loop on 0")

        # move to the next position
        if magnitude > delta:
            boundaryBreaks += 1
            print("bb rot", delta, magnitude)
        elif dial == 0:
            zeroes += 1
            print("on zero")

        dial += magnitude * direction
        dial %= 100

    print(f"{zeroes=}")
    print(f"{zeroes+boundaryBreaks=}")

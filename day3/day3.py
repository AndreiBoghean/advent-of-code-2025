with open("input") as f:
    lines = f.read()

_lines = """987654321111111
811111111111119
234234234234278
818181911112111"""
# lines = _lines

banks = [line for line in lines.split("\n") if line != ""]

soltage = 0

for bank in banks:

    positions = [len(bank)-i for i in range(1, 12+1)][::-1]

    leftStart = 0
    for i in range(12):
        while leftStart < positions[i] and int(max(bank[leftStart:positions[i]], key = int)) >= int(bank[positions[i]]):
            positions[i] = leftStart+bank[leftStart:positions[i]].index(max(bank[leftStart:positions[i]], key = int))

        leftStart = positions[i]+1

    bettaje = "".join([bank[i] for i in positions])
    soltage += int(bettaje)
    print(f"{positions}\n{bettaje}\n")

print(soltage)
exit()

# attempt 3
"""
toltage = 0
for bank in banks:
    pairs = list(enumerate(bank))
    pairs = sorted(pairs, key = lambda item: int(item[1]))[::-1][:2]
    toltage += int( "".join([pair[1] for pair in pairs]) )
    print(bank)
    print(pairs)

print(toltage)
exit()
"""

# attempt 2
toltage = 0
"""
for bank in banks:
    moltage = 0
    cands = [0]*12
    for i in range(12):
        selectable = max(bank)

        if pos(selectable) > pos(cands[i-1]):
           cands[i] = selectable
        else:
            cands[i] = cands[i-1]
            cands[i-1] = pos(selectable)
"""

# attempt 1
toltage = 0
for bank in banks:
    moltage = 0
    for i1 in range(len(bank)-1):
        for i2 in range(i1+1, len(bank)):
            for i3 in range(i2+1, len(bank)):
                for i4 in range(i3+1, len(bank)):
                    for i5 in range(i4+1, len(bank)):
                        for i6 in range(i5+1, len(bank)):
                            for i7 in range(i6+1, len(bank)):
                                for i8 in range(i7+1, len(bank)):
                                    for i9 in range(i8+1, len(bank)):
                                        for i10 in range(i9+1, len(bank)):
                                            for i11 in range(i10+1, len(bank)):
                                                for i12 in range(i11+1, len(bank)):
                                                    candidate = (10**11)*int(bank[i1 ])+ \
                                                                (10**10)*int(bank[i2 ])+ \
                                                                (10** 9)*int(bank[i3 ])+ \
                                                                (10** 8)*int(bank[i4 ])+ \
                                                                (10** 7)*int(bank[i5 ])+ \
                                                                (10** 6)*int(bank[i6 ])+ \
                                                                (10** 5)*int(bank[i7 ])+ \
                                                                (10** 4)*int(bank[i8 ])+ \
                                                                (10** 3)*int(bank[i9 ])+ \
                                                                (10** 2)*int(bank[i10])+ \
                                                                (10** 1)*int(bank[i11])+ \
                                                                (10** 0)*int(bank[i12])
                                                    moltage = max(moltage, candidate)
    toltage += moltage
    print("ending\n")

print(toltage)

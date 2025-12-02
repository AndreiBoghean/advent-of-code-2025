with open("input") as f:
    lines = f.read()

# lines = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"

ranges = [(int(range.split("-")[0]), int(range.split("-")[1])) for range in lines.split(",")]

invalids = []

for r1, r2 in ranges:
    print(str(r1).ljust(10), str(r2).ljust(10), end=" ")
    for candidint in range(r1, r2+1):
        candidate = str(candidint)

        for substr in [candidate[:i] for i in range(1, len(candidate))]:
            if len(candidate) % len(substr) != 0: continue

            # uncomment to solve for part 1 of the challenge; keep commented to solve part 2.
            # if len(candidate) // len(substr) != 2: continue

            if substr * (len(candidate) // len(substr)) == candidate:
                invalids.append(candidint)
                break
    print(invalids)

print(invalids)
print(sum(invalids))

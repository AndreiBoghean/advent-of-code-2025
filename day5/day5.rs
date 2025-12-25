use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _test_str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    let _actual_str = fs::read_to_string("input")?;
    let action_str = _actual_str.trim_end();

    let portions: Vec<&str> = action_str.split("\n\n").collect();

    let mut ranges: Vec<[u64; 2]> = Vec::new();
    for line in portions[0].split("\n") {
        let pair: Vec<&str> = line.split("-").collect();
        ranges.push([pair[0].parse().unwrap(), pair[1].parse().unwrap()]);
        // println!("strt:{} end:{}", ranges[ranges.len()-1][0], ranges[ranges.len()-1][1])
    }
    ranges.sort_by(|&a, &b| a[0].cmp(&b[0]));
    println!("we have {} raw ranges {:?}", ranges.len(), ranges);

    let mut canIDs: Vec<u64> = Vec::new();
    for line in portions[1].split("\n") {
        canIDs.push(line.parse().unwrap());
    }
    canIDs.sort();


    // simplify ranges attempt 2
    // we need to create a new range store and overwrite the old one, because idk how to
    // double-loop over the same data in rust. (cant consume 2 iters... cant double consume
    // same iter...)
    let mut new_ranges: Vec<[u64; 2]> = Vec::new();

    let mut i = 0;
    while i < ranges.len() {
        let mut range = ranges[i];
        // println!("i at {}\n", i);
        // println!("range {}:{}", range[0], range[1]); // range a now ends at range b end.

        let mut i2 = i+1;
        while i2 < ranges.len() {
            // if range[1] >= ranges[i2][0] && range[1] <= ranges[i2][1] // end of range a is inside range b.
            // { println!("range now {}:{}", range[0], ranges[i2][1]); } // range a now ends at range b end.
            // else if range[1] >= ranges[i2][1] // range a engulfs range b.
            // { println!("{}:{} >= {}:{}", range[0], range[1], ranges[i2][0], ranges[i2][1]); } // range b is wholely ignored.
            // else // we cant merge this range.
            // { println!("jupming to {}", i2); } // we set our header to the distinct range, and repeat merging process.

            if range[1] >= ranges[i2][0] && range[1] <= ranges[i2][1] // end of range a is inside range b.
            { range[1] = ranges[i2][1] } // range a now ends at range b end.
            else if range[1] < ranges[i2][1] // we cant merge this range, because there's a gap
            { break; } // we set our header to the distinct range, and repeat merging process.
            // else // implicit secret case for when range a engulfs range b.
            // { } // range b is completely skipped from the merging, and the i2 pointer advances over it.

            i2 += 1;
        }

        i = i2;
        // println!("pushing {}:{}", range[0], range[1]); // range a now ends at range b end.
        new_ranges.push(range);
    }
    ranges = new_ranges;

    // // simplify ranges attempt 1
    // 'sipler: loop {
    //     // try to find two simplifiable pairs
    //     for i1 in 0..ranges.len() {
    //         let mut range_a = ranges[i1];

    //         for i2 in (i1+1)..ranges.len() {
    //             let range_b = ranges[i2];

    //             if range_a[1] > range_b[0] && range_a[0] < range_b[0] { // a is before b, and overlaps. (i.e. b start is in the middle of a)
    //                 print!("1simplified {}:{} and {}:{}", range_a[0], range_a[1], range_b[0], range_b[1]);
    //                 range_a[1] = range_b[1];
    //                 new_ranges.push()
    //             }
    //             else if range_a[0] <= range_b[1] && range_a[1] >= range_b[1] { // a is after b, and overlaps. (i.e. b end is in the middle of a)
    //                 print!("2simplified {}:{} and {}:{}", range_a[0], range_a[1], range_b[0], range_b[1]);
    //                 range_a[0] = range_b[0];
    //                 new_ranges.push([range_b[0], range_a])
    //             }
    //             else { // no overlap detected so dont delete item b. note that we dont add it to
    //                    // new_ranges at this point because the outer loop will eventually deal with
    //                    // it.
    //                 continue;
    //             }

    //             println!(" | now {}:{}", range_a[0], range_a[1]);

    //             // ranges.remove(i2);
    //             continue 'sipler;
    //         }
    //         new_ranges.push(range_a); // we keep the range, but only if it didnt get combined with
    //                                   // anything.
    //     }

    //     break;
    // }

    ranges.sort_by(|&a, &b| a[0].cmp(&b[0])); // technically, merging should preserve the sort by
                                              // range start, but this is here just to be safe.
    println!("we have {} joined ranges {:?}", ranges.len(), ranges);


    // counting fresh items attempt 2 - for each ID, find if there is a range it's inside of.
    // unimplemented microoptimisation: since ranges are merged, finding a gap between ranges within which the current ID lies should allow early termination.
    let mut freshes = 0;
    for canID in canIDs {
        for range in &ranges {
            if canID >= range[0] && canID <= range[1] {
                freshes += 1;
                break;
            }
        }
    }

    let mut allowables = 0;
    for range in ranges {
        allowables += range[1] - range[0] + 1
    }

    // counting logic for fresh items attempt 1 - for each range, find all IDs inside it and increment.
    // // count fresh ingredients
    // let mut freshes = 0;
    // for range in ranges {
    //     for id in range[0]..(range[1]+1) {
    //         while canIDs.contains(&id) {
    //             if let Some(id_pos) = canIDs.iter().position(|&x| x == id)
    //                 { canIDs.swap_remove(id_pos); }
    //             freshes += 1;
    //         }
    //     }
    // }

    println!("fresh ingredients: {}", freshes);
    println!("allowables: {}", allowables);
    Ok(())
}

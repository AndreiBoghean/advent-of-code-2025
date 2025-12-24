use std::fs;
use std::error::Error;


fn collect_rolls(data: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
    let mut new_data: Vec<Vec<i32>> = Vec::new();
    let mut accessible = 0;

    for i in 0..data.len() {
        let mut row_vec: Vec<i32> = Vec::new();

        for j in 0..data[i].len() {
            let sm = (if i >= 1 {
                    (if j >= 1 {data[i-1][j-1]} else {0}) +
                    data[i-1][j+0] +
                    (if j+1 <  data[0].len() {data[i-1][j+1]} else {0})
                } else {0}) +

                (if j >= 1 {data[i+0][j-1]} else {0}) +
                // data[i+0][j+0] + // note: dont count ourselves in the roll count lol
                (if j+1 <  data[0].len() {data[i+0][j+1]} else {0}) +

                (if i+1 < data.len() {
                    (if j >= 1 {data[i+1][j-1]} else {0}) +
                    data[i+1][j+0] +
                    (if j+1 <  data[0].len() {data[i+1][j+1]} else {0})
                } else {0});

            if data[i][j] == 1 && sm < 4 { // if on a removable roll..
                print!("x");
                row_vec.push(0); // indicate roll removal by pushing an empty space
            }
            else {
                print!("{}", data[i][j]);
                row_vec.push(data[i][j]); // keep the existing item (empty space "." or roll "@")
            }
            // print!("{}", _data[i][j]);
            // print!("{}", data[i][j]);

            if data[i][j] == 1 && sm < 4 { accessible += 1; }
        }

        new_data.push(row_vec);
        println!();
        }
    
    return (new_data, accessible);
}

fn main() -> Result<(), Box<dyn Error>> {
    let _test_str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let actual_str = fs::read_to_string("input")?;
    
    // let action_str = _test_str;
    let action_str = actual_str.trim_end();

    let data_str: Vec<Vec<char>> = action_str.split("\n").map(|s| s.chars().collect()).collect();
    let data: Vec<Vec<i32>> = action_str.split("\n").map(|s| s.chars().map(|c| if c == '@' {1} else {0}).collect()).collect();

    let (mut new_data, mut accessible) = collect_rolls(data);

    let mut removed_rolls = 0;
    removed_rolls += accessible;

    while accessible > 0
    {
        (new_data, accessible) = collect_rolls(new_data);
        removed_rolls += accessible;
        println!("just removed: {}\n", accessible);

    }

    println!("total removed: {}", removed_rolls);

    Ok(())
}

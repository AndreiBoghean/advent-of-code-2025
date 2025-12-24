use std::fs;
use std::error::Error;

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

    let mut neighbours: Vec<Vec<i32>> = Vec::new();
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

            print!("{}", if data_str[i][j] == '@' && sm < 4 {'x'} else {data_str[i][j]});
            // print!("{}", _data[i][j]);
            // print!("{}", data[i][j]);

            row_vec.push(sm);
            if data_str[i][j] == '@' && sm < 4 { accessible += 1; }
        }

        neighbours.push(row_vec);
        println!();
        }


    println!("Hello, world!");
    println!("accesible: {}", accessible);

    Ok(())
}

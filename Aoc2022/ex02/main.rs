/*
* A/X is rock
* B/Y is paper
* C/Z is scissors
* 
* A win is 6 points
* A draw is 3 points
* A loss is 0
*
* The objective is to calculate the final score.
*
*/

use std::io::Error;
use std::fs;

//first solution by timvisee
/*
* first solution:
* println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b + 3 * (1 + b - a).rem_euclid(3))
            .sum::<i16>(),
    );
* second solution:
*  println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b * 3 + (2 + a + b) % 3)
            .sum::<i16>(),
    );
*
*
*
*
*/
//second solution by erning
/*
fn part_one(input: &str) -> i32{
    input.lines()
    .map(|v| v.as_bytes())
    .map(|v| match (v[0], v[2]) {
            (b'A', b'X') => 4, // 1 + 3
            (b'A', b'Y') => 8, // 2 + 6
            (b'A', b'Z') => 3, // 3 + 0
            (b'B', b'X') => 1, // 1 + 0
            (b'B', b'Y') => 5, // 2 + 3
            (b'B', b'Z') => 9, // 3 + 6
            (b'C', b'X') => 7, // 1 + 6
            (b'C', b'Y') => 2, // 2 + 0
            (b'C', b'Z') => 6, // 3 + 3
            _ => panic!(),
        })
        .sum()
}

fn part_two(input: &str) -> i32{
 input
        .lines()
        .map(|v| v.as_bytes())
        .map(|v| match (v[0], v[2]) {
            (b'A', b'X') => 3, // 3 + 0
            (b'A', b'Y') => 4, // 1 + 3
            (b'A', b'Z') => 8, // 2 + 6
            (b'B', b'X') => 1, // 1 + 0
            (b'B', b'Y') => 5, // 2 + 3
            (b'B', b'Z') => 9, // 3 + 6
            (b'C', b'X') => 2, // 2 + 0
            (b'C', b'Y') => 6, // 3 + 3
            (b'C', b'Z') => 7, // 1 + 6
            _ => panic!(),
        })
        .sum()
}

fn read_as_string(filename: &str) -> Result<String, Error> {
    return Ok(fs::read_to_string(filename)?);
}

fn main() -> Result<(), std::io::Error> {
    let content = read_as_string("input")?;
    let result = part_two(&content);
    println!("{}", result);
    Ok(())
}
*/

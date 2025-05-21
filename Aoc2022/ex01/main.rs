use std::fs;
use std::io::Error;

/* PART ONE *
fn is_line_empty(s: &str) -> bool{
    if s.trim().is_empty(){
        true
    }else {
        false
    }
}

fn main() -> std::io::Result<()>{
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    let mut max_calories = 0;
    let mut current_elf = 0;
    for line in content.lines(){
        if is_line_empty(line){
            if current_elf > max_calories{
                max_calories = current_elf;
            }
            current_elf = 0;
        }
        current_elf += line.parse().unwrap_or(0);
    }
    if current_elf > max_calories{
        max_calories = current_elf;
    }
    println!("{}", max_calories);
    Ok(())
}*/

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.split("\n\n")
        .map(|g| g.trim().lines().map(|s| s.parse().unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> i32{
    parse_input(input)
        .into_iter()
        .map(|g| g.into_iter().sum())
        .max()
        .unwrap()
}
pub fn part_two(input: &str) -> i32 {
    let mut calories: Vec<i32> = parse_input(input)
        .into_iter()
        .map(|g| g.into_iter().sum())
        .collect();
    calories.sort_unstable();
    calories.into_iter().rev().take(3).sum()
}

fn read_as_string(filename: &str) -> Result<String, Error> {
    return Ok(fs::read_to_string(filename)?);
}

fn main() -> Result<(), std::io::Error> {
    let input = read_as_string("input")?;
    let result = part_two(&input);
    println!("{}", result);
    Ok(())
}





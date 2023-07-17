use std::{fs::File, io::{Result, BufReader, BufRead}};




fn main() -> Result<()> {
    let path = "./src/input.txt";
    let target = 2020;
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let ln:Vec<i32> = reader.lines()
          .filter(|i| i.is_ok())
          .map(|i| i.unwrap())
          .map(|i| i.parse::<i32>())
          .filter(|i| i.is_ok())
          .map(|i| i.unwrap())
          .collect();
    //aoc2020-part1
    // for i in 0..ln.len(){
    //     for j in (i+1)..ln.len()  {
    //         if ln[j] == target - ln[i] {
    //             let res = ln[i] * ln[j];
    //             println!("{:?}",res);
    //
    //         }
    //     }
    // }
    //aoc2020-part2

    for i in 0..ln.len(){
        for j in (i+1)..ln.len()  {
            for k in i+2..ln.len(){
                if ln[j] == target - (ln[i] + ln[k]) {
                    let res = ln[i] * ln[j] * ln[k];
                    println!("{:?}",res);

                }
            }
        }
    }




    Ok(())
}

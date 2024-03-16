use core::panic;
use std::env::args;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Error, Write};

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if !(args.len() > 1) {
        panic!("No file path passed");
    }

    let lottery_numbers_file_path = &args[1];
    let lottery_numbers_file = File::open(lottery_numbers_file_path)?;
    let lottery_numbers_buffer = BufReader::new(lottery_numbers_file);

    let mut lottery_numbers: Vec<[u8; 5]> = vec![];
    for number_sequence in lottery_numbers_buffer.lines() {
        let mut sequence: [u8; 5] = [0; 5];
        for (index, number) in number_sequence?.split(" ").into_iter().enumerate() {
            sequence[index] = number.parse().unwrap();
        }
        lottery_numbers.push(sequence);
    }

    let mut final_week: [u8; 5] = [0; 5];

    for index in 0..final_week.len() {
        println!("{}. szám:", index + 1);
        let mut number = String::new();
        stdin().read_line(&mut number)?;

        while number.lines().next().unwrap().parse::<u8>().unwrap() > 90
            || 1 > number.lines().next().unwrap().parse::<u8>().unwrap()
        {
            println!("{}. szám:", index + 1);
            number = String::new();
            stdin().read_line(&mut number)?;
        }

        final_week[index] = number.lines().next().unwrap().parse::<u8>().unwrap();
    }

    lottery_numbers.push(final_week);

    println!("");
    println!("A hét sorszáma:");
    let mut week_index_string = String::new();
    stdin().read_line(&mut week_index_string)?;

    let mut week_index: u8 = week_index_string.lines().next().unwrap().parse().unwrap();

    while week_index - 1 > (lottery_numbers.len() - 1) as u8 {
        println!("A hét sorszáma:");
        week_index_string = String::new();
        stdin().read_line(&mut week_index_string)?;
        week_index = week_index_string.lines().next().unwrap().parse().unwrap();
    }

    let mut chosen_week_string = String::new();
    for number in lottery_numbers[(week_index - 1) as usize] {
        chosen_week_string = format!("{}{} ", chosen_week_string, number);
    }

    println!("{}", chosen_week_string);

    let mut occurences_of_42: u16 = 0;
    let mut occurences_of_odd_numbers: u16 = 0;

    for week in &lottery_numbers {
        occurences_of_42 += (week.iter().filter(|&x| *x == 42).count()) as u16;
        occurences_of_odd_numbers += (week.iter().filter(|&x| *x % 2 == 1).count()) as u16;
    }

    println!("");
    println!(
        "A 42-es számot {} alkalommal húzták ki az 52 hét során.",
        occurences_of_42
    );

    println!("");
    println!(
        "Az 52 hét során {} alkalommal húztak ki páratlan számot.",
        occurences_of_odd_numbers
    );

    let mut statistics: [u16; 90] = [0; 90];

    for num in 0..90 {
        for week in &lottery_numbers {
            let amount = week.iter().filter(|&x| *x == num + 1).count() as u16;
            statistics[num as usize] += amount;
        }
    }

    let statistics_file_path = "statisztika.txt";
    let mut statistics_file = File::create(statistics_file_path)?;
    for (index, amount) in statistics.iter().enumerate() {
        write!(statistics_file, "{}: {}db\n", index + 1, amount)?;
    }

    Ok(())
}

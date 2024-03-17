use core::panic;
use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Error, Write};

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if !(args.len() > 1) {
        panic!("No file path passed");
    }

    let answers_file_path = &args[1];
    let answers_file = File::open(answers_file_path)?;
    let answers_buffer = BufReader::new(answers_file);

    let mut correct_answers = String::new();

    let mut participant_answers: HashMap<String, String> = HashMap::new();

    for (index, answer) in answers_buffer.lines().into_iter().enumerate() {
        let temp_answer = answer?;
        if index == 0 {
            correct_answers = temp_answer
                .clone()
                .split_whitespace()
                .next()
                .unwrap()
                .to_string();
        } else {
            let mut participant_answer: Vec<String> = vec![];
            for i in temp_answer.clone().split_whitespace() {
                participant_answer.push(i.to_string());
            }
            participant_answers
                .insert(participant_answer[0].clone(), participant_answer[1].clone());
        }
    }

    println!(
        "A vetélkedőn {} versenyző indult.",
        participant_answers.len()
    );

    println!("");
    println!("A versenyző azonosítója:");
    let mut participant = String::new();
    stdin().read_line(&mut participant)?;
    participant = participant.lines().next().unwrap().to_string();

    println!("{}", participant_answers[&participant]);

    let mut answer_string = String::new();

    for (index, letter) in participant_answers[&participant]
        .chars()
        .into_iter()
        .enumerate()
    {
        if letter == correct_answers.chars().nth(index).unwrap() {
            answer_string = format!("{}{}", answer_string, "+".to_string());
        } else {
            answer_string = format!("{}{}", answer_string, " ".to_string());
        }
    }

    println!("{}", answer_string);

    println!("");
    println!("A feladat sorszáma:");
    let mut task_number_input = String::new();
    stdin().read_line(&mut task_number_input)?;
    let task_number: u8 = task_number_input.lines().next().unwrap().parse().unwrap();

    let mut number_of_correct_answers: u16 = 0;

    for (_, value) in &participant_answers {
        if value.chars().nth((task_number - 1).into()).unwrap()
            == correct_answers
                .chars()
                .nth((task_number - 1).into())
                .unwrap()
        {
            number_of_correct_answers += 1;
        }
    }

    let correct_percentage = (number_of_correct_answers as u32 * 10000
        / participant_answers.len() as u32) as f32
        / 100.0;

    println!("");
    println!(
        "A feladatra {} fő, a versenyzők {}%-a adott helyes választ.",
        number_of_correct_answers, correct_percentage
    );

    println!("");
    println!("A versenyző kódja:");
    participant = String::new();
    stdin().read_line(&mut participant)?;
    participant = participant.lines().next().unwrap().to_string();

    let mut participant_points: u8 = 0;
    let mut participant_points_string = String::new();

    if participant_answers.contains_key(&participant) {
        for (index, letter) in participant_answers[&participant]
            .chars()
            .into_iter()
            .enumerate()
        {
            if letter == correct_answers.chars().nth(index).unwrap() {
                participant_points += match index {
                    0..=4 => 3,
                    5..=9 => 4,
                    10..=12 => 5,
                    13 => 6,
                    _ => 0,
                }
            }
        }

        participant_points_string = format!("{} {} pont", participant, participant_points);
    } else {
        participant_points_string =
            String::from("A versenyen nem vett rész ilyen kódszámú versenyző.");
    }

    let participant_points_file_path = "pontszam.txt";
    let mut participant_points_file = File::create(participant_points_file_path)?;
    write!(participant_points_file, "{}", participant_points_string)?;

    Ok(())
}

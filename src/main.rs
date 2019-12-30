#![allow(clippy::unreadable_literal, clippy::zero_prefixed_literal)]
use std::{
    fmt,
    cmp,
    io
};
use rand::prelude::*;
use rayon::prelude::*;

const N: usize = 1_000_000;
const SAMPLE_SIZE: usize = 750;

#[derive(Copy, Clone, Debug)]
pub struct Code([u8; 6]);
impl Code {
    fn new(mut n: u32) -> Self {
        let mut arr = [0_u8; 6];
        for digit in &mut arr {
            *digit = (n % 10) as u8;
            n /= 10;
        }
        Self(arr)
    }
}
impl fmt::Display for Code {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for digit in self.0.iter().rev() {
            write!(formatter, "{}", *digit)?;
        }
        Ok(())
    }
}

fn main() {

    let mut possible = (0..(N as u32))
        .map(Code::new)
        .collect::<Vec<Code>>();
    let mut done = false;
    while possible.len() != 1 {
        let guess = if possible.len() == N {Code::new(112233)}
        else {get_best_guess(&possible)};

        println!("{}", guess);
        // println!("!{:02}", get_response(guess, Code::new(123456)));

        let response = translate(prompt());
        if response == translate(60) {
            done = true;
        }
        let mut i = 0;
        while i < possible.len() {
            if translate(get_response(guess, possible[i])) != response {
                possible.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }
    if !done {
        println!("{}", possible[0]);
        prompt();
    }
    println!("I guessed it!");
}

fn prompt() -> u8 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<u8>().unwrap()
}

fn get_best_guess(possible: &[Code]) -> Code {
    let mut rng = SmallRng::from_entropy();
    let sample = possible
        .choose_multiple(&mut rng, cmp::min(SAMPLE_SIZE, possible.len()))
        .copied()
        .collect::<Vec<Code>>();

    let scores = sample.par_iter().map(|guess| {
        let mut freq = [0_u32; 28];
        for answer in &sample {
            freq[translate(get_response(*guess, *answer)) as usize] += 1;
        }

        let mut elim = [0_u32; 28];
        for response in 0..28 {
            for answer in &sample {
                if translate(get_response(*guess, *answer)) != response {
                    elim[response as usize] += 1;
                }
            }
        }

        (*guess, freq.iter()
            .zip(elim.iter())
            .map(|(f, e)| *f * *e)
            .sum::<u32>())
    });

    scores.max_by_key(|(_, score)| *score)
        .unwrap().0
}

fn translate(input: u8) -> u8 {
    let sum = input % 10;
    let exact = input / 10;
    let exact = 6 - exact;
    exact * (exact + 1) / 2 + sum
}


fn get_response(guess: Code, answer: Code) -> u8 {
    let mut exact = 0;
    let mut count_guess = [0_u8; 10];
    let mut count_answer = [0_u8; 10];
    for i in 0..6 {
        if guess.0[i] == answer.0[i] {
            exact += 1;
        } else {
            count_guess[guess.0[i] as usize] += 1;
            count_answer[answer.0[i] as usize] += 1;
        }
    }
    let mut sum = 0;
    for i in 0..10 {
        sum += cmp::min(count_answer[i], count_guess[i]);
    }
    10 * exact + sum
}

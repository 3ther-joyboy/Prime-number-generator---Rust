use std::time::Instant;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::read_to_string;
use std::fs::OpenOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "primes.csv";
    let file: File;

    let mut index: u32 = 3;
    let mut primes: Vec<u32> = vec![3];

    if Path::new(file_name).exists() {
        load(&mut primes,&mut index,&file_name);
        file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_name)
                .expect("Failed to open file");
    }else{
        file = File::create(file_name)?;
    }

    search_for_primes(file,primes, index);
    Ok(())
}

fn column_from_csv(line: String, mut column: u32) -> String {
    column += 1; // To start from 0
    let bytes = line.as_bytes();
    let mut start = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b',' {
            column -= 1;
            if column == 0 {
                return line[start..i].to_string();
            }
            start = i+1; // Excludes "," character
        }

    }
    line[start..].to_string() // Returns last column
}
fn load( primes: &mut Vec<u32>,index: &mut u32, filename: &str) {
    for line in read_to_string(filename).unwrap().lines() {
        let prime = column_from_csv(line.to_string(),0).parse().unwrap();
        *index = prime;
        primes.push(prime);
    }
}
fn search_for_primes(mut file: File,mut primes: Vec<u32>,mut index: u32) {
    let mut last_prime = Instant::now();

    loop {
        index+=2;
        let mut divaidible = false;

        for prime in &primes {
            if index%prime == 0 {
                divaidible = true;
                break;
            }
        }
        if !divaidible {
            primes.push(index);
            let time = last_prime.elapsed().as_secs_f64()*1000.0;



            let _ = file.write({
                            format!("{},{:?}\n",index.to_string(), time)
                        }.as_bytes());
            print!("\rPrime: {}\t Time since last: [ms] {:?}\t\t Count: {}",index.to_string(),time,primes.len());

            last_prime = Instant::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_search_first() {
        let line: String = String::from("42,ahoj,404");
        assert_eq!(column_from_csv(line,0),"42");
    }
    #[test]
    fn csv_search_middle() {
        let line: String = String::from("42,ahoj,404");
        assert_eq!(column_from_csv(line,1),"ahoj");
    }
    #[test]
    fn csv_search_last() {
        let line: String = String::from("42,ahoj,404");
        assert_eq!(column_from_csv(line,2),"404");
    }


}

use::std::prelude::*;
use flate2::Compress;
use flate2::read::GzDecoder;
use::rand::prelude::*;
use::rand::Rng;
use std::io;
use std::fs::{File, remove_file};
use std::io::{Read, BufWriter, Write, BufReader, BufRead};
use std::path::Path;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;

const CAPACITY: usize = 10240;
fn compress(file_path: &Path, password: &str) {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut buffer = Vec::new();
    buffer.write_all(password.as_bytes()).expect("Failed to write pass");
    encoder.write_all(&buffer).expect("Failed to write compressed data to buffer");
    let compressedBytes = encoder.finish().unwrap();
    let compressedFilePath = file_path.with_extension(".txt.gz");
    let mut compressedFile =
        BufWriter::new(File::create(&compressedFilePath).expect("Failed to create file"));
    compressedFile
        .write_all(&compressedBytes)
        .expect("Failed to write compressed data to file");
    remove_file(file_path).expect("Failed to delete original file");
}
fn decompress(file_path: &Path){
    let instream = File::open(file_path).unwrap();
    let decode = GzDecoder::new(instream);
    let mut outstream = BufReader::with_capacity(CAPACITY, decode);
    let mut contents = String::new();
    outstream.read_to_string(&mut contents);
    let password = contents.lines().next().unwrap_or("");
    println!("{}", password);    
  
}
fn main() {
    
    let letters: &[&str] = &[
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"
    ];

    let numbers: &[&str] = &[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

    let symbols: &[&str] = &[
        "?", "/", ">", ".", "<", ",", ":", ";", "@", "~", "#", "}", "]", "[", "=", "-", "+", "_", ")", "(", "*", "&", "^", "%", "$", "£"
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: Program <filename>");
        return;
    }
    let mut filePath = args[1].clone();
    println!("{}", filePath);
    let mut passArr = Vec::new();
    let mut randVal = rand::thread_rng();
    println!("Would you like to generate a password or retreive one?");
    println!("0 to generate and 1 to retreive: ");
    let mut decision = String::new();
    io::stdin().read_line(&mut decision).expect("failure to read input");
    
       // println!("{}", assert_eq!(decision, no));
    
    
    for _ in 0..23 {
        let randChar = letters[randVal.gen_range(0..letters.len())];
        let randNum = numbers[randVal.gen_range(0..numbers.len())]; 
        let randSymbol = symbols[randVal.gen_range(0..symbols.len())];
        let rValue = [randChar, randNum, randSymbol][randVal.gen_range(0..3)];
        passArr.push(rValue);
    
    }
    let password = passArr.join("");
    

    if decision == "0" {
        println!("kek");
        
        let fPath = Path::new(&filePath);
        
        println!("fille path:: {}", filePath);
        let mut file = BufWriter::new(File::create(&filePath).expect("File Creation Failed"));
        file.write_all(password.as_bytes()).expect("Failed to write password to file");
        compress(&fPath , &password);

    }else{

        filePath.push_str(".txt.gz");
        let cPath = Path::new(&filePath);
        println!("{}", filePath);
        decompress(&cPath);
       
    }; 


   
    
       }


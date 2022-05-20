use anyhow::{ Result};
use cli_lang_tool::{Dictionary, Word};
use std::{fs::File, io::{prelude::*, BufReader}, path::PathBuf};
use serde::{Serialize, Deserialize};
use rand::Rng;
use structopt::StructOpt;


const FILE_PATH: &str = "./examples/storage/turk.json";


fn main() {
    println!("Hello, sozluk");
}



/// Try create sozluk from the file or (if can`t) create new sozluk instance
fn sozluk_init() -> Sozluk {
    let backup_path: PathBuf = FILE_PATH.into();
    let sozluk = match Sozluk::try_from_file(backup_path) {
        Ok(sozluk) => sozluk,
        Err(_) => Sozluk::new(),
    };
    sozluk
}


#[derive(Serialize, Deserialize, Clone)]

pub struct Sozluk {
    records: Vec<Word>
}

impl Sozluk {
    /// Create an empty Sozluk instance.
    fn new() -> Self {
        Sozluk {
            records: Vec::new(),
        }
    }

    /// Try to build a dictionary from a `file_path`; usually you'd want to error out
    /// or init an empty dictionary with `Sozluk::new()`.
    fn try_from_file(file_path: PathBuf) -> Result<Self> {
        let file = File::open(file_path.clone())?;
        let reader = BufReader::new(file);
        let this = serde_json::from_reader(reader).unwrap();//map_err(|_| anyhow!("Unable to read file {}", file_path.display()))?;

        Ok(this)
    }

    fn get_rand_word(&self) -> Option<&Word> {
        self.records.get(rand::thread_rng().gen_range(0..=&self.count()-1))   
    }
}
   

impl Drop for Sozluk {
    fn drop(&mut self) {
        let file_path: PathBuf = FILE_PATH.into();
        let mut file = File::create(&file_path).expect("Unable to open a backup file");
        file.write_all(serde_json::to_string(&self).unwrap().as_bytes()).unwrap();
    }
}

impl Dictionary for Sozluk {
    const LANG: &'static str = "TR";

    fn count(&self) -> usize {
        self.records.len()
    }

    fn add_word(
        &mut self,
        native: String,
        english: String,
        categories: Vec<String>,
        examples: Vec<String>,
    ) -> Result<()> {
        self.records
            .push(Word::new(native, english, categories, examples));
        Ok(())
    }

    fn get_word(&self, word: &String) -> Option<&Word> {
        self.records.iter().find(|w| &w.native == word)
    }

    fn get_word_en(&self, en_word: &String) -> Option<&Word> {
        self.records.iter().find(|w| &w.english == en_word)
    }

    fn get_all(&self) -> &Vec<Word> {
        &self.records
    }
}
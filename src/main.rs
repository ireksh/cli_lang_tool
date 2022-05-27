use anyhow::{Result};
use clli::{Dictionary, Word};
use std::{fs::File, io::{prelude::*, BufReader}, path::PathBuf};
use serde::{Serialize, Deserialize};
use rand::Rng;
use clap::{ Parser, Subcommand};


const FILE_PATH: &str = "./examples/storage/turk.json";

#[derive(Debug, Parser)]
#[clap(name = "clli")]
#[clap(about = "turkish lang tool", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // #[clap(arg_required_else_help = true)]
    ///Get the random word from the sozluk
    Random{
        /// count - how many random words
        #[clap(default_value_t = 1)]
        count: u8,
    },
    #[clap(arg_required_else_help = true)]
    /// Adds records to the dictionary
    Add
        {
        #[clap(required = true)]
        native: String,
        english: String,
        },

    /// Get the count of words in the dictionary 
    Count,

    /// Get the all words from the dictionaryy
    GetAllWords,

    /// Translate english -> turkish
    Etranslate{
        #[clap(required = true)]
        english: String,
    },

    /// Translate turkish -> english 
    Ttranslate{
        #[clap(required = true)]
        native: String,
    },    
}

fn main() {
    let args = Cli::parse();
    let mut sozluk = sozluk_init();

    match args.command {
        Commands::Random{ count}  => {
            for _ in 0..count {
                if let Some(word) = sozluk.get_rand_word() {
                    println!("The random word : {}. translate:{}", &word.native, &word.english );
                    }
                }
            }
        Commands::Add { native , english } => {
            println!("Pushing to {} :: {}", native , english);
            let dummy_cat: Vec<String> = vec![];
            let dummy_examples: Vec<String> = vec![];
            match sozluk.add_word(native, english, dummy_cat, dummy_examples){
                Ok(()) => println!("New record added"),
                Err(err) => println!("Error {}", err),     
            } 
        }
        Commands::Count => {
            println!("{} records in dictionary", sozluk.count());
        }
        Commands::Etranslate {english} => {
            if let Some(word) = sozluk.get_word(&english) {
                println!("{} = translate = {} ", word.native, &english);    
            } else {
                println!("Translate for {} not found",&english);
            }
        }    
        Commands::Ttranslate {native} => {
            if let Some(word) = sozluk.get_word_en(&native) {
                println!("{} = translate = {} ", word.english, &native,);    
            } else {
                println!("Translate for {} not found", &native);
            }
        }
        Commands::GetAllWords =>{
            let mut msg = String::new();
            for rec in sozluk.get_all().iter(){
                let _ = &msg.push_str(format!("Turkish: {}, transtlate: {}, examples: {} \n", rec.native, &rec.english, &rec.examples.join("; ")).as_str());
            }
            println!("sozluk {}",&msg);
        }
     }    
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


#[derive(Serialize, Deserialize,)]

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

    fn get_word(&self, native: &String) -> Option<&Word> {
        self.records.iter().find(|w| &w.native == native)
    }

    fn get_word_en(&self, en_word: &String) -> Option<&Word> {
        self.records.iter().find(|w| &w.english == en_word)
    }

    fn get_all(&self) -> &Vec<Word> {
        &self.records
    }
}

// #[cfg(test)]


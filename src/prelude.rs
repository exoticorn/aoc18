use quicli::prelude::*;
use std::cell::Cell;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

pub use regex::Regex;

pub enum Answer {
    Usize(usize),
    I32(i32),
}

impl From<usize> for Answer {
    fn from(v: usize) -> Answer {
        Answer::Usize(v)
    }
}

impl From<i32> for Answer {
    fn from(v: i32) -> Answer {
        Answer::I32(v)
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Answer::Usize(v) => write!(f, "{}", v),
            Answer::I32(v) => write!(f, "{}", v),
        }
    }
}

pub struct Answers(Answer, Option<Answer>);

impl fmt::Display for Answers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "First answer: {}", self.0)?;
        if let Some(ref bonus) = self.1 {
            write!(f, ", second answer: {}", bonus)?;
        }
        Ok(())
    }
}

pub type AocResult = Result<Answers>;

#[allow(dead_code)]
pub fn answer<T: Into<Answer>>(v: T) -> AocResult {
    Ok(Answers(v.into(), None))
}

#[allow(dead_code)]
pub fn answers<A: Into<Answer>, B: Into<Answer>>(a: A, b: B) -> AocResult {
    Ok(Answers(a.into(), Some(b.into())))
}

pub struct AocData {
    path: PathBuf,
    error: Cell<Option<Error>>,
}

impl AocData {
    pub fn new(day: usize) -> AocData {
        AocData {
            path: format!("data/{:02}.txt", day).into(),
            error: Cell::new(None),
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> Result<String> {
        read_file(&self.path)
    }

    #[allow(dead_code)]
    pub fn lines(&self) -> Result<AocLines> {
        Ok(AocLines {
            file: BufReader::new(File::open(&self.path)?),
            error: &self.error,
        })
    }

    pub fn ok(self) -> Result<()> {
        if let Some(err) = self.error.into_inner() {
            return Err(err);
        }
        Ok(())
    }
}

pub struct AocLines<'a> {
    file: BufReader<File>,
    error: &'a Cell<Option<Error>>,
}

impl<'a> Iterator for AocLines<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut result = String::new();
        match self.file.read_line(&mut result) {
            Ok(0) => None,
            Ok(_) => {
                let len = result.trim_right().len();
                result.truncate(len);
                Some(result)
            }
            Err(err) => {
                self.error.set(Some(err.into()));
                None
            }
        }
    }
}

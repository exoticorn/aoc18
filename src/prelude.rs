pub use quicli::prelude::*;
use std::cell::Cell;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::str::FromStr;

pub use regex::Regex;

pub enum Answer {
    Usize(usize),
    I32(i32),
    U32(u32),
    String(String),
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

impl From<u32> for Answer {
    fn from(v: u32) -> Answer {
        Answer::U32(v)
    }
}

impl From<String> for Answer {
    fn from(v: String) -> Answer {
        Answer::String(v)
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Answer::Usize(v) => write!(f, "{}", v),
            Answer::I32(v) => write!(f, "{}", v),
            Answer::U32(v) => write!(f, "{}", v),
            Answer::String(ref v) => write!(f, "{}", v),
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
        let mut s = read_file(&self.path)?;
        s.truncate(s.trim_right().len());
        Ok(s)
    }

    #[allow(dead_code)]
    pub fn lines(&self) -> Result<AocLines<String>> {
        self.values()
    }

    #[allow(dead_code)]
    pub fn values<T: FromStr>(&self) -> Result<AocLines<T>> {
        Ok(AocLines {
            file: BufReader::new(File::open(&self.path)?),
            error: &self.error,
            buffer: String::new(),
            t: PhantomData,
        })
    }

    pub fn ok(self) -> Result<()> {
        if let Some(err) = self.error.into_inner() {
            return Err(err);
        }
        Ok(())
    }
}

pub struct AocLines<'a, T: FromStr> {
    file: BufReader<File>,
    error: &'a Cell<Option<Error>>,
    buffer: String,
    t: PhantomData<T>,
}

impl<'a, T: FromStr> Iterator for AocLines<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.buffer.clear();
        match self.file.read_line(&mut self.buffer) {
            Ok(0) => None,
            Ok(_) => {
                let trimmed = self.buffer.trim_right();
                match trimmed.parse() {
                    Ok(r) => Some(r),
                    Err(_) => {
                        self.error
                            .set(Some(format_err!("Failed to parse '{}'", trimmed)));
                        None
                    }
                }
            }
            Err(err) => {
                self.error.set(Some(err.into()));
                None
            }
        }
    }
}

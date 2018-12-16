#![allow(dead_code)]

pub use quicli::prelude::*;
use std::cell::Cell;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Cursor};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::str::FromStr;

pub use regex::Regex;

pub struct Answer(String);

impl From<usize> for Answer {
    fn from(v: usize) -> Answer {
        Answer(v.to_string())
    }
}

impl From<isize> for Answer {
    fn from(v: isize) -> Answer {
        Answer(v.to_string())
    }
}

impl From<i32> for Answer {
    fn from(v: i32) -> Answer {
        Answer(v.to_string())
    }
}

impl From<u32> for Answer {
    fn from(v: u32) -> Answer {
        Answer(v.to_string())
    }
}

impl From<u8> for Answer {
    fn from(v: u8) -> Answer {
        Answer(v.to_string())
    }
}

impl From<String> for Answer {
    fn from(v: String) -> Answer {
        Answer(v)
    }
}

impl<A: Into<Answer>, B: Into<Answer>> From<(A, B)> for Answer {
    fn from(v: (A, B)) -> Answer {
        Answer(format!("({}, {})", v.0.into(), v.1.into()))
    }
}

impl<A: Into<Answer>, B: Into<Answer>, C: Into<Answer>> From<(A, B, C)> for Answer {
    fn from(v: (A, B, C)) -> Answer {
        Answer(format!("({}, {}, {})", v.0.into(), v.1.into(), v.2.into()))
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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

pub fn answer<T: Into<Answer>>(v: T) -> AocResult {
    Ok(Answers(v.into(), None))
}

pub fn answers<A: Into<Answer>, B: Into<Answer>>(a: A, b: B) -> AocResult {
    Ok(Answers(a.into(), Some(b.into())))
}

enum DataSrc {
    Path(PathBuf),
    Str(&'static str),
}

pub struct AocData {
    data: DataSrc,
    error: Cell<Option<Error>>,
}

impl AocData {
    pub fn new(day: usize) -> AocData {
        AocData {
            data: DataSrc::Path(format!("data/{:02}.txt", day).into()),
            error: Cell::new(None),
        }
    }

    pub fn from_str(s: &'static str) -> AocData {
        AocData {
            data: DataSrc::Str(s.trim()),
            error: Cell::new(None),
        }
    }

    pub fn to_string(&self) -> Result<String> {
        let mut s = match self.data {
            DataSrc::Path(ref path) => read_file(path)?,
            DataSrc::Str(s) => s.to_string(),
        };
        s.truncate(s.trim_right().len());
        Ok(s)
    }

    pub fn lines(&self) -> Result<AocLines<String>> {
        self.values()
    }

    pub fn values<T: FromStr>(&self) -> Result<AocLines<T>> {
        Ok(AocLines {
            file: match self.data {
                DataSrc::Path(ref path) => Box::new(BufReader::new(File::open(path)?)),
                DataSrc::Str(s) => Box::new(Cursor::new(s)),
            },
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
    file: Box<dyn BufRead>,
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

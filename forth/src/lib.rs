use itertools::Itertools;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::SplitWhitespace;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    runtime_stack: Vec<Value>,
    defs: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Default::default()
    }

    pub fn stack(&self) -> Vec<Value> {
        self.runtime_stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let input = input.to_uppercase();
        self.parse(&mut input.split_whitespace().peekable())
    }

    fn parse(&mut self, tokens: &mut Peekable<SplitWhitespace>) -> ForthResult {
        match tokens.next() {
            None => Ok(()),
            Some(token) => {
                match token {
                    ":" => self.parse_def(tokens)?,
                    "+" if !self.defs.contains_key(token) => self.add()?,
                    "-" if !self.defs.contains_key(token) => self.sub()?,
                    "*" if !self.defs.contains_key(token) => self.mul()?,
                    "/" if !self.defs.contains_key(token) => self.div()?,
                    "DUP" if !self.defs.contains_key(token) => self.dup()?,
                    "DROP" if !self.defs.contains_key(token) => self.drop()?,
                    "SWAP" if !self.defs.contains_key(token) => self.swap()?,
                    "OVER" if !self.defs.contains_key(token) => self.over()?,
                    _ => self.parse_value(token)?,
                }
                self.parse(tokens)
            }
        }
    }

    fn parse_def(&mut self, tokens: &mut Peekable<SplitWhitespace>) -> ForthResult {
        let word = tokens.next().ok_or(Error::InvalidWord)?;
        if word.chars().all(|c| c.is_digit(10)) {
            return Err(Error::InvalidWord);
        }
        let def_body: Vec<String> = tokens
            .take_while_ref(|&w| w != ";")
            .map(|s| s.to_string())
            .collect();

        match tokens.peek() {
            Some(&s) if s == ";" => {
                tokens.next();
                let body = def_body
                    .into_iter()
                    .flat_map(|w| {
                        if let Some(v) = self.defs.get(&w.to_string()) {
                            v.clone()
                        } else {
                            vec![w]
                        }
                    })
                    .collect::<Vec<String>>();
                self.defs.insert(word.to_string(), body);
                Ok(())
            }
            _ => Err(Error::InvalidWord),
        }
    }

    fn add(&mut self) -> ForthResult {
        let val = self.pop()? + self.pop()?;
        self.push(val);
        Ok(())
    }

    fn sub(&mut self) -> ForthResult {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(b - a);
        Ok(())
    }

    fn mul(&mut self) -> ForthResult {
        let val = self.pop()? * self.pop()?;
        self.push(val);
        Ok(())
    }

    fn div(&mut self) -> ForthResult {
        let a = self.pop()?;
        let b = self.pop()?;
        if a == 0 {
            return Err(Error::DivisionByZero);
        }
        self.push(b / a);
        Ok(())
    }

    fn dup(&mut self) -> ForthResult {
        let val = self.pop()?;
        self.push(val);
        self.push(val);
        Ok(())
    }

    fn drop(&mut self) -> ForthResult {
        self.pop()?;
        Ok(())
    }

    fn swap(&mut self) -> ForthResult {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(a);
        self.push(b);
        Ok(())
    }

    fn over(&mut self) -> ForthResult {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(b);
        self.push(a);
        self.push(b);
        Ok(())
    }

    fn parse_value(&mut self, val: &str) -> ForthResult {
        match val.parse::<Value>() {
            Ok(num) => self.push(num),
            _ if !self.defs.contains_key(&val.to_string()) => return Err(Error::UnknownWord),
            _ => {
                let mut forth = Forth::new();
                forth.set_stack(self.stack());
                let code = self.defs.get(val).ok_or(Error::UnknownWord)?;
                forth.eval(&code.join(" "))?;
                self.set_stack(forth.stack());
            }
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<Value, Error> {
        self.runtime_stack.pop().ok_or(Error::StackUnderflow)
    }

    fn push(&mut self, val: Value) {
        self.runtime_stack.push(val);
    }

    fn set_stack(&mut self, stack: Vec<Value>) {
        self.runtime_stack = stack
    }
}

use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

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
        Forth {
            runtime_stack: Vec::new(),
            defs: HashMap::new(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.runtime_stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        println!("eval (input={})", input);
        println!("stack = {:?}", self.runtime_stack);
        println!("defs = {:?}", self.defs);
        let input = input.to_uppercase();
        let tokens = input.split_whitespace();
        let mut inside_def = false;
        let mut def_body : Vec<String> = Vec::new();
        for token in tokens {
            if inside_def {
                if token == ";" {
                    inside_def = false;
                    let word = &def_body.first().ok_or(Error::StackUnderflow)?;
                    if word.chars().all(|c| c.is_digit(10)) {
                        return Err(Error::InvalidWord);
                    }
                    let body = def_body.clone().into_iter().skip(1)
                    .flat_map(|w| {
                        if let Some(v) = self.defs.get(&w.to_string()) {
                                v.clone()
                        } else {
                            vec![w.to_string()]
                        }
                    })
                    .collect::<Vec<String>>();

                    self.defs.insert(word.to_string(), body);
                    println!("definio: {} como {:?}", word, def_body);
                    println!("defs = {:?}", self.defs);
                } else {
                    def_body.push(token.to_string());
                }
            } else {
                match token {
                    ":"  => {
                        inside_def = true;
                        def_body = Vec::new();
                    }
                    "+" if !self.defs.contains_key(token)=> {
                        let val = self.pop()? + self.pop()?;
                        self.push(val);
                    }
                    "-" if !self.defs.contains_key(token) => {
                        let a = self.pop()?;
                        let b = self.pop()?;
                        self.push(b - a);
                    }
                    "*" if !self.defs.contains_key(token)=> {
                        let val = self.pop()? * self.pop()?;
                        self.push(val);
                    }
                    "/" if !self.defs.contains_key(token) => {
                        let a = self.pop()?;
                        let b = self.pop()?;
                        if a == 0 {
                            return Err(Error::DivisionByZero);
                        }
                        self.push(b / a);
                    }
                   
                    "DUP" if !self.defs.contains_key(token) => {
                        let val = self.pop()?;
                        self.push(val);
                        self.push(val);
                    }
                    "DROP" if !self.defs.contains_key("DROP") => {
                        self.pop()?;
                    }
                    "SWAP" if !self.defs.contains_key("SWAP")=> {
                        let a = self.pop()?;
                        let b = self.pop()?;
                        self.push(a);
                        self.push(b);
                    }
                    "OVER" if !self.defs.contains_key("OVER") => {
                        let a = self.pop()?;
                        let b = self.pop()?;
                        self.push(b);
                        self.push(a);
                        self.push(b);
                    }
                    val  => {
                        println!("val is {}", val);
                     match val.parse::<Value>() {
                        Ok(num) => self.push(num),
                        _ => {
                            let key = val.to_uppercase();
                            if !self.defs.contains_key(&key)  {
                                return Err(Error::UnknownWord)
                            } else {
                                let mut forth = Forth::new();
                                forth.set_stack(self.stack());
                                let code = self.defs.get(val).ok_or(Error::UnknownWord)?;
                                let eval_code = &code.join(" ");
                                println!("eval code = {}", eval_code);
                                forth.eval(&code.join(" "))?;
                                self.set_stack(forth.stack());
                            }
                        },
                    }},
                }
            }
        }
        if inside_def {
            return Err(Error::InvalidWord); 
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<Value, Error> {
        self.runtime_stack.pop().ok_or(Error::StackUnderflow)
    }

    fn push(&mut self, val: Value) {
        println!("push {}", val);
        self.runtime_stack.push(val);
    }

    fn set_stack(&mut self, stack: Vec<Value>) {
        self.runtime_stack = Vec::from(stack)
    }

}

use std::io;

#[derive(Debug)]
enum Kind {
   Number,
   Operator(String),
}

#[derive(Debug)]
struct Token {
    kind: Kind,
    val: Option<f32>,
}

struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream { tokens }
    }

    pub fn get(&mut self) -> Token {
       self.tokens.remove(0) 
    }

    pub fn status(&self) {
        println!("{:?}", self.tokens);
    }

    pub fn put_back(&mut self, t: Token) {
        self.tokens.insert(0, t);
    }
}


fn expression(ts: &mut TokenStream) -> f32 {
    let mut left = term(ts); // read and evaluate the expression
    let mut t = ts.get(); // get the next token

    loop {
        match t.kind {
            Kind::Operator(o) => {
                match o.as_str() {
                    "+" => {
                        left += term(ts);
                        t = ts.get();
                        break;
                    },
                    "-" => {
                        left -= term(ts);
                        t = ts.get();
                        break;
                    },
                    _ => panic!("Unknown Operator!"),
                }
            },
            _ => {
                ts.put_back(t);
                return left;
            },
        }
    }

   left 
}

fn term(ts: &mut TokenStream) -> f32 {
    let mut left = primary(ts);
    let mut t = ts.get();
    
    loop {
        match t.kind {
            Kind::Operator(o) => {
                match o.as_str() {
                    "*" => {
                        left *= primary(ts);
                        t = ts.get();
//                        break;
                    },
                    "/" => {
                        let d = primary(ts);
                        if d == 0.0 {
                            panic!("divide by zero :(");
                        }
                        left /= d;
                        t = ts.get();
 //                       break;
                    },
                    _ => panic!("Unknown Operator"),
                }
            },
            _ => {
                ts.put_back(t);
                return left;
            }
        }
    }
}

fn primary(ts: &mut TokenStream) -> f32 {
    let mut t = ts.get();
    match t.kind {
        Kind::Operator(o) => {
            match o.as_str() {
                "(" => {
                    let d = expression(ts);
                    t = ts.get();
                    match t.kind {
                        Kind::Operator(o) => {
                            match o.as_str() {
                                ")" => return d,
                                _ => panic!("expected )"),
                            }
                        },
                        _ => (),
                    }
                },
                _ => 0.0,
            }
        },
        Kind::Number => return t.val.unwrap(),
        _ => panic!("Expected Primary"),
    }
}

fn main() -> io::Result<()> {
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let mut stream: Vec<&str> = Vec::new();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match buffer.trim() {
            "" => {break;},
            _ => {
                for s in buffer.trim().split(" ") {
                    stream.push(s);
                }
            },
        }

        for token in &stream {
            match token.parse::<f32>() {
                Ok(n) => tokens.push(Token { kind: Kind::Number, val: Some(n) }),
                Err(_e) => tokens.push(Token { 
                    kind: Kind::Operator(String::from(*token)), 
                    val: None 
                }),
            }
        }
    }

    let mut ts = TokenStream::new(tokens);
    let mut val = 0.0;
    loop {
        let t = ts.get();

        match t.kind {
            Kind::Operator(ref o) => {
                match o.as_str() {
                   "q" => break,
                   ";" => println!("={}\n", val),
                   _ => ts.put_back(t),
                }
            },
            _ => (),
        }

        val = expression(&mut ts);
    }
    
    Ok(())
}

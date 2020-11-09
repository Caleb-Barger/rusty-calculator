use std::io;

struct TokenStream {
    buffer: Option<Token>,
    full : bool,
}

impl TokenStream {
    pub fn new() -> Self {
        TokenStream { buffer: None, full: false }
    }

    pub fn put_back(&mut self, t: Token) {
        if self.full {
            panic!("put_back call into a full buffer");
        }
        self.buffer = Some(t);
        self.full = true;
    }

    pub fn get(&mut self) -> Token {
        if full {
            self.full = false;
            self.buffer
        }
    }
}

struct Token<'a> {
    pub kind: &'a str,
    pub val: Option<i32>,
}

// functions to match the calculator grammer
    // get_token
    // expression
    // term
    // primary

fn expression() -> f32 {
    let mut left = term(); // read and evaluate the expression
    let mut t = ts.get(); // get the next token

    loop {
        match t.kind {
            "+" => {
                left += term();
                t = ts.get();
                break;
            },
            "-" => {
                left -= term();
                t = ts.get();
                break;
            },
            _ => {
                ts.put_back(t);
                return left;
            },
        }
    }
}

fn term() -> f32 {
    let mut left = primary();
    let mut t = ts.get();
    
    loop {
        match t.kind {
            "*" => {
                left *= primary();
                t = ts.get();
                break;
            },
            "/" => {
                let d = primary();
                if d == 0 {
                    panic!("divide by zero :(");
                }
                left /= d;
                t = ts.get();
                break;
            },
            _ => {
                ts.put_back(t);
                return left;
            }
        }
    }
}

fn primary() -> f32 {
    let mut t = ts.get();
    match t.kind {
        "(" => {
            let d = expression();
            t = ts.get();
            if t.kind != ")" {
                panic!("expected )");
            }
            return d;
        },
        "8" => {
            return t.val;
        },
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
    let mut val = 0;
    loop {
        let t = ts.get();

        if t.kind == "q" {
            break;
        }

        if t.kind == ";" {
            println!("={}\n", val);
        }

        else {
            ts.put_back(t);
        }

        val = expression();
    }
    
    Ok(())
}


use std::io;

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

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let v: Vec<&str> = input.trim().split(' ').collect();
    let mut tokens: Vec<Token> = Vec::new();

    for val in &v {
        println!("{}", expression());
    }
    
    Ok(())
}

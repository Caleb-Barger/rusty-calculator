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
    let mut t = get_token(); // get the next token

    loop {
        match t.kind {
            "+" => {
                left += term();
                t = get_token();
                break;
            },
            "-" => {
                left -= term();
                t = get_token();
                break;
            },
            _ => return left,
        }
    }
}

fn term() -> f32 {
    let mut left = primary();
    let mut t = get_token();
    
    loop {
        match t.kind {
            "*" => {
                left *= primary();
                t = get_token();
                break;
            },
            "/" => {
                let d = primary();
                if d == 0 {
                    panic!("divide by zero :(");
                }
                left /= d;
                t = get_token();
                break;
            },
            _ => return left,
        }
    }
}

fn primary() -> f32 {
    let mut t = get_token();
    match t.kind {
        "(" => {
            let d = expression();
            t = get_token();
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
        match val.parse::<i32>() {
            Ok(n) => {
                tokens.push(Token { kind: "8", val: Some(n) });
            },
            Err(_e) => {
                tokens.push(Token { kind: val, val: None });
            },
        }
    }
    
    let mut total = 0;
    let mut should_add = false;

    for t in &tokens {
        match t.val {
            Some(v) => println!("value of token - {}", v),
            None => println!("No value for token"),
        }

        match t.kind {
            "+" => should_add = true, 
            _ => (), 
        }
        println!("Token kind - {}", t.kind);
    }

    if should_add {
        total = &tokens[0].val.unwrap() + &tokens[2].val.unwrap(); 
    }

    println!("{}", total);


    Ok(())
}

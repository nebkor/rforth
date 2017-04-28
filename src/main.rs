use std::collections::HashMap;

type TOK = String;
type DICT = HashMap<TOK, Word>;
type STACK = Vec<TypedToken>;

// #[derive(Clone)]
enum Word {
    Primitive(Box<Fn(&mut STACK)>),
    Compound(STACK),
}

#[derive(Clone, Debug)]
enum TypedToken {
    Word(TOK),
    // Str(String),
    Num(f64),
}

fn main() {
    let mut n_stack = STACK::new();
    let mut n_dict = DICT::new();

    let n_dup = Word::Primitive(Box::new(|mut s: &mut STACK| {
        let t = s[0].clone();
        s.push(t);
    }));

    let n_star = Word::Primitive(Box::new(|mut s: &mut STACK| {
        let p1 = s.pop().unwrap();
        let p2 = s.pop().unwrap();
        let v1: f64;
        let v2: f64;

        match p1 {
            TypedToken::Num(v) => {
                v1 = v;
            }
            _ => panic!(),
        };
        match p2 {
            TypedToken::Num(v) => {
                v2 = v;
            }
            _ => panic!(),
        };

        let prod = v1 * v2;
        s.push(TypedToken::Num(prod));
    }));

    n_dict.insert("DUP".to_string(), n_dup);

    n_dict.insert("*".to_string(), n_star);

    n_stack = process_input("2 DUP *".to_string(), n_stack, n_dict);

    println!("stack is now {:?}", n_stack);

}

fn process_word(mut stack: &mut STACK, word: Word) {
    match word {
        Word::Primitive(p) => {
            p(&mut stack);
        }
        Word::Compound(mut c) => {
            stack.append(&mut c);
        }
    }
}

fn mk_typedtoken(t: String) -> TypedToken {
    let n = t.parse::<f64>();
    match n {
        Ok(n) => TypedToken::Num(n),
        Err(_) => TypedToken::Word(t),
    }
}

fn process_tokens(input: &STACK, mut stack: &mut STACK, mut dict: &mut DICT) {
    for i in input.into_iter() {
        process_token(i, stack, dict);
    }
}

fn process_token(input: &TypedToken, mut stack: &mut STACK, mut dict: &mut DICT) {
    match input.clone() {
        TypedToken::Num(n) => {
            stack.push(input.clone());
        }
        TypedToken::Word(w) => {
            let ref dw: Word = *dict.get(&w).unwrap();
            match dw {
                &Word::Primitive(ref p) => {
                    p(stack);
                }
                &Word::Compound(ref c) => {
                    process_tokens(&c, stack, dict);
                }
            }
        }
    }
}

fn process_input(input: String, mut stack: STACK, mut dict: DICT) -> STACK {
    let toks = input.split_whitespace().map(|x| mk_typedtoken(x.to_string())).collect::<STACK>();

    process_tokens(&toks, &mut stack, &mut dict);

    stack.clone()
}

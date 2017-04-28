use std::collections::HashMap;

type TOK = String;
type DICT = HashMap<TOK, Word>;
type STACK = Vec<TypedToken>;

type Pdict = HashMap<String, Box<Fn(&mut STACK)>>;

#[derive(Clone)]
enum Word {
    Primitive(String),
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

    let mut p_dict = Pdict::new();

    let p_dup = Box::new(|mut s: &mut STACK| {
        let t = s[0].clone();
        s.push(t);
    });

    let p_star = Box::new(|mut s: &mut STACK| {
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
    });

    // let p_colon = Box::new(|mut s: &mut)

    let dup_string = "DUP".to_string();
    let star_string = "*".to_string();

    p_dict.insert(dup_string.clone(), p_dup);
    n_dict.insert(dup_string.clone(), Word::Primitive(dup_string));

    p_dict.insert(star_string.clone(), p_star);
    n_dict.insert(star_string.clone(), Word::Primitive(star_string));

    let square_stack: STACK = vec!["DUP".to_string(), "*".to_string()]
        .iter()
        .map(|x| mk_typedtoken(x.to_string()))
        .collect();

    n_dict.insert("SQUARED".to_string(), Word::Compound(square_stack));

    n_stack = process_input("2 DUP * SQUARED".to_string(), n_stack, n_dict, &p_dict);

    println!("stack is now {:?}", n_stack);
}


fn mk_typedtoken(t: String) -> TypedToken {
    let n = t.parse::<f64>();
    match n {
        Ok(n) => TypedToken::Num(n),
        Err(_) => TypedToken::Word(t),
    }
}

fn process_tokens(input: &STACK, mut stack: &mut STACK, mut dict: &mut DICT, pmap: &Pdict) {
    for i in input.into_iter() {
        process_token(i, stack, dict, pmap);
    }
}

fn process_token(input: &TypedToken, mut stack: &mut STACK, mut dict: &mut DICT, pmap: &Pdict) {
    match input.clone() {
        TypedToken::Num(_) => {
            stack.push(input.clone());
        }
        TypedToken::Word(w) => {
            let dw: Word = dict.get(&w).unwrap().clone();
            match dw {
                Word::Primitive(ref p) => {
                    (pmap.get(p).unwrap())(stack);
                }
                Word::Compound(ref c) => {
                    process_tokens(&c, stack, dict, pmap);
                }
            }
        }
    }
}

fn process_input(input: String, mut stack: STACK, mut dict: DICT, pmap: &Pdict) -> STACK {
    let toks = input.split_whitespace().map(|x| mk_typedtoken(x.to_string())).collect::<STACK>();

    process_tokens(&toks, &mut stack, &mut dict, pmap);

    stack.clone()
}

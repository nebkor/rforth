#![feature(inclusive_range_syntax)]

use std::collections::HashMap;

type TOK = String;
type Fdict = HashMap<TOK, DictEntry>;
type Fstack = Vec<TypedToken>;

type Pdict = HashMap<String, Box<Fn(&mut Fstack)>>;

// struct ForthDict {
//     fdict: Fdict,
//     pdict: Pdict,
// }

#[derive(Clone)]
enum DictEntry {
    Primitive(String),
    Compound(Fstack),
    Str(String),
}

#[derive(Clone, Debug, PartialEq)]
enum TypedToken {
    Entry(TOK),
    Num(f64),
}


fn main() {
    let mut n_stack = Fstack::new();
    let mut n_dict = Fdict::new();

    let mut p_dict = Pdict::new();

    let p_dup = Box::new(|mut s: &mut Fstack| {
        let t = s[0].clone();
        s.push(t);
    });

    let p_star = Box::new(|mut s: &mut Fstack| {
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


    let dup_string = "DUP".to_string();
    let star_string = "*".to_string();

    p_dict.insert(dup_string.clone(), p_dup);
    n_dict.insert(dup_string.clone(), DictEntry::Primitive(dup_string));

    p_dict.insert(star_string.clone(), p_star);
    n_dict.insert(star_string.clone(), DictEntry::Primitive(star_string));

    let square_stack: Fstack = vec!["DUP", "*"]
        .iter()
        .map(|x| mk_typedtoken(x))
        .collect();

    n_dict.insert("SQUARED".to_string(), DictEntry::Compound(square_stack));

    n_stack = process_input("2 DUP * SQUARED", n_stack, n_dict, &p_dict);

    println!("stack is now {:?}", n_stack);
}


fn mk_typedtoken(t: &str) -> TypedToken {
    let n = t.parse::<f64>();
    match n {
        Ok(n) => TypedToken::Num(n),
        Err(_) => TypedToken::Entry(t.to_owned()),
    }
}

fn process_tokens(input: Fstack, mut stack: &mut Fstack, mut dict: &mut Fdict, pmap: &Pdict) {
    for i in input.iter() {
        process_token(&i, stack, dict, pmap);
    }
}

fn process_token(input: &TypedToken, mut stack: &mut Fstack, mut dict: &mut Fdict, pmap: &Pdict) {
    match *input {
        TypedToken::Num(_) => {
            stack.push(input.clone());
        }
        TypedToken::Entry(ref w) => {
            let dw = dict.get(w).cloned();
            match dw {
                Some(DictEntry::Primitive(ref p)) => {
                    (pmap.get(p).unwrap())(stack);
                }
                Some(DictEntry::Compound(ref c)) => {
                    process_tokens(c.clone(), stack, dict, pmap);
                }
                Some(DictEntry::Str(_)) => {
                    stack.push(input.clone());
                }
                None => {
                    match dict.insert(w.clone(), DictEntry::Str(w.clone())) {
                        _ => {}
                    }
                }
            }
        }
    }
}

fn add_word(def: Fstack, dict: &mut Fdict) {
    let words: Fstack = def[1..def.len()].iter().map(|x| x.clone()).collect();
    let name = words[0].clone();
    let name = match name {
        TypedToken::Entry(w) => w.clone(),
        _ => "".to_string(),
    };

    dict.insert(name, DictEntry::Compound(words));
}

fn process_input(input: &str, mut stack: Fstack, mut dict: Fdict, pmap: &Pdict) -> Fstack {
    let mut toks = input.split_whitespace().map(|x| mk_typedtoken(x)).collect::<Fstack>();

    let colon_tok = TypedToken::Entry(":".to_owned());
    let exit_tok = TypedToken::Entry("EXIT".to_owned());

    while let Some(i) = toks.iter().position(|x| x == &colon_tok) {
        let j = toks.iter().position(|x| x == &exit_tok);
        match j {
            Some(j) => {
                let def = toks.drain(i...j).collect();
                add_word(def, &mut dict);
            }
            None => panic!(),
        }
    }

    {
        process_tokens(toks, &mut stack, &mut dict, pmap);
    }
    stack
}

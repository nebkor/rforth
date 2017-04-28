use std::collections::HashMap;

type TOK = String;
type DICT = HashMap<TOK, Word>;
type STACK = Vec<TOK>;

struct Word {
    immediate: bool,
    pub cp: Box<Fn(STACK) -> STACK>,
}


fn main() {
    let mut n_stack = STACK::new();
    let mut n_dict = DICT::new();

    let n_dup = Word {
        immediate: false,
        cp: Box::new(|mut s| {
            let t = s[0].clone();
            s.push(t);
            s
        }),
    };

    let n_star = Word {
        immediate: false,
        cp: Box::new(|mut s| {
            let p1 = s.pop().unwrap();
            let p2 = s.pop().unwrap();
            s.push(format!("{}",
                           p1.parse::<f32>().unwrap() * p2.parse::<f32>().unwrap()));
            s
        }),
    };

    n_dict.insert("DUP".to_string(), n_dup);

    n_dict.insert("*".to_string(), n_star);

    n_stack = run("2 DUP *".to_string(), &mut n_stack, &mut n_dict);

    println!("stack is now {:?}", n_stack);

}

fn run(input: String, stack: &mut STACK, dict: &mut DICT) -> STACK {
    let toks = input.split_whitespace().collect::<Vec<_>>();

    for t in toks.into_iter() {
        if dict.contains_key(t) {
            *stack = (dict.get(t).unwrap().cp)(stack.clone());
        } else {
            stack.push(t.to_string());
        }
    }

    stack.clone()
}

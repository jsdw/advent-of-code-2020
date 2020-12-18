use structopt::StructOpt;
use shared::{ FileContentOpts };
use std::iter;

fn main() -> Result<(),anyhow::Error> {
    let opts = FileContentOpts::from_args();

    let lines: Vec<_> = opts.file
        .lines()
        .filter_map(|l| treeize_tokens(&tokenize_expr(l)))
        .collect();

    let sum: usize = lines
        .iter()
        .filter_map(|l| calculate_part1(l))
        .sum();
    println!("Star 1: {}", sum);

    let sum: usize = lines
        .into_iter()
        .filter_map(calculate_part2)
        .sum();
    println!("Star 2: {}", sum);

    Ok(())
}

fn calculate_part1(tree: &[TokenTree]) -> Option<usize> {
    let mut total = 0;
    let mut op = Op::Add;
    for tok in tree.iter() {
        match tok {
            TokenTree::Digit(n) => {
                match op {
                    Op::Add => total += *n,
                    Op::Mult => total *= *n
                }
            },
            TokenTree::Tree(tree) => {
                let n = calculate_part1(&tree)?;
                match op {
                    Op::Add => total += n,
                    Op::Mult => total *= n
                }
            },
            TokenTree::Op(new_op) => {
                op = *new_op
            }
        }
    }
    Some(total)
}

fn calculate_part2(mut tree: Vec<TokenTree>) -> Option<usize> {
    fn get_digit(t: &TokenTree) -> Option<usize> {
        match t {
            TokenTree::Digit(n) => Some(*n),
            TokenTree::Tree(t) => calculate_part2(t.clone()),
            _ => None
        }
    }
    fn find_op(tree: &[TokenTree], search_op: Op) -> Option<(usize,Op)> {
        tree
            .iter()
            .enumerate()
            .filter_map(|t| t.1.get_op().map(|o| (t.0,o)))
            .find(|(_,op)| *op == search_op)
    }
    fn collapse_one(mut tree: Vec<TokenTree>) -> Option<Vec<TokenTree>> {
        let (idx,op) = find_op(&tree, Op::Add).or(find_op(&tree, Op::Mult))?;
        let collapsed = {
            let n1 = get_digit(tree.get(idx-1)?)?;
            let n2 = get_digit(tree.get(idx+1)?)?;
            let res = match op {
                Op::Add => n1 + n2,
                Op::Mult => n1 * n2
            };
            TokenTree::Digit(res)
        };
        tree.splice(idx-1..=idx+1, iter::once(collapsed));
        Some(tree)
    }

    while tree.len() > 1 {
        tree = collapse_one(tree)?;
    }
    get_digit(tree.get(0)?)
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Token {
    LeftParen,
    RightParen,
    Digit(usize),
    Op(Op)
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
enum Op {
    Add,
    Mult
}

fn tokenize_expr(s: &str) -> Vec<Token> {
    s.chars().filter_map(|c| {
        if let Some(d) = c.to_digit(10) {
            Some(Token::Digit(d as usize))
        } else {
            match c {
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '+' => Some(Token::Op(Op::Add)),
                '*' => Some(Token::Op(Op::Mult)),
                _ => None
            }
        }
    }).collect()
}

#[derive(Clone,Debug,PartialEq,Eq)]
enum TokenTree {
    Tree(Vec<TokenTree>),
    Digit(usize),
    Op(Op)
}

impl TokenTree {
    fn get_op(&self) -> Option<Op> {
        match self {
            TokenTree::Op(op) => Some(*op),
            _ => None
        }
    }
}

fn treeize_tokens(toks: &[Token]) -> Option<Vec<TokenTree>> {
    fn find_right_paren_idx(toks: &[Token]) -> Option<usize> {
        let mut c = 0;
        for (idx, &t) in toks.iter().enumerate() {
            if t == Token::LeftParen { c += 1 }
            else if t == Token::RightParen && c > 0 { c -= 1 }
            else if t == Token::RightParen { return Some(idx) }
        }
        None
    }

    let mut tree = Vec::new();
    let mut idx = 0;
    while idx < toks.len() {
        let (new_idx, item) = match toks[idx] {
            Token::Digit(n) => (idx + 1, TokenTree::Digit(n)),
            Token::Op(op) => (idx + 1, TokenTree::Op(op)),
            Token::LeftParen => {
                let new_idx = find_right_paren_idx(&toks[idx+1..])? + idx + 1;
                let inner_toks = treeize_tokens(&toks[idx+1..new_idx])?;
                (new_idx+1, TokenTree::Tree(inner_toks))
            },
            Token::RightParen => {
                /* We should never see these; leftparen will handle them */
                return None
            }
        };
        idx = new_idx;
        tree.push(item);
    }
    Some(tree)
}
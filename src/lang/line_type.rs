use std::collections::HashMap;
use crate::lang::line_type::LineType::{If, Nothing, Out, VarOperate};
use crate::lang::tokenizer::{Compare, Keyword, Operation, Token};
use crate::lang::tokenizer::Token::Name;

#[derive(Debug)]
pub enum LineType {
    Nothing,
    Out(Vec<String>),
    VarOperate(String, Operation, Vec<Token>), // (var_name, operator, expression)
    If(Vec<Token>, Compare, Vec<Token>, Vec<Vec<Token>>), // (expression, comparator, expression, lines
    While(Vec<Token>, Compare, Vec<Token>, Vec<Vec<Token>>),
    //Function, TODO
}

pub fn construct_line_type(tokens: Vec<Token>) -> LineType {
    let tokens = tokens;
    if tokens.len() == 0 {return Nothing  }

    if let Token::Name(name) = tokens.get(0).unwrap() {
        // Check if its a print "a: b: c:" <=> print(a,b,c) "a:" <=> print(a)
        if tokens.len() == 1 {
            return Out(vec![name.clone()]);
        } else {
            let mut outs = Vec::new();
            for chunk in tokens[1..].chunks_exact(2) {
                if let [Token::Colon, Token::Name(name2)] = chunk {
                    outs.push(name2.clone())
                }
            }
            if !outs.is_empty() {
                outs.insert(0, name.clone());
                return Out(outs)
            }
        }
        // Check for var assignment
        if tokens.len() >= 3 {
            match tokens.get(1).unwrap() {
                Token::Operation(o) =>  {
                    return VarOperate(name.clone(), o.clone(), tokens[2..].to_owned())
                }
                _ => {}
            }
        }
    }

    if let Token::Key(keyword) = tokens.get(0).unwrap() {
        match keyword {
            Keyword::If => {
                let mut tmp: Option<(usize, Compare)> = None;
                for i in 1..tokens.len() {
                    if let Token::Compare(compare) = tokens.get(i).unwrap() {
                        tmp = Some((i, compare.clone()))
                    }
                }
                let (i, comp) = tmp.expect("No comparator in if statement");

                let mut limit: Option<usize> = None;
                for i in i+1..tokens.len() {
                    if let Token::Colon = tokens.get(i).unwrap() {
                        limit = Some(i);
                        break;
                    }
                }
                let limit = limit.expect("No consequence to while");
                let splits = Vec::from(tokens.get(limit+1..tokens.len()).unwrap()).split(|x| x.clone() == Token::Colon)
                    .filter(|&subvec| subvec != [Token::Colon])
                    .map(|subvec| subvec.to_vec())
                    .collect();
                return If(Vec::from(tokens.get(1..i).unwrap()), comp, Vec::from(tokens.get(i+1..limit).unwrap()), splits)
            }
            Keyword::While => {} // TODO
        }
    }

    return Nothing

    ;
}

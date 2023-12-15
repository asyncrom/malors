// use std::collections::HashMap;
// use std::thread;
// use crate::lang::calculator::{Expression, Possible};
// use crate::lang::runner::Possible::{PossExpression, PossToken};
// use crate::lang::tokenizer::{Operation, Operator, post_process_operation, Token, tokenize};
// use crate::lang::tokenizer::Token::{Name, Number};
//
// pub fn compile_line(memory: &mut HashMap<String, f64>, tokens: Vec<Token>) {
//     //TODO implement functions
//     let mut tokens = tokens;
//     post_process_operation(&mut tokens);
//     println!("after process: {:?}", tokens.clone());
//     let three = three_composer(tokens);
//     if let Possible::PossExpression(expression) = three {
//         let result = expression.solve(memory);
//         println!("result: {}", result)
//     }
//     //println!("three: {:?}", three);
// }
//
// fn var(memory: &mut HashMap<&str, f64>, tokens: Vec<Token>) {
//
// }
//
// fn three_composer(tokens: Vec<Token>) -> Possible {
//     let mut version: (Vec<Token>, Operator, Vec<Token>) = (vec![], Operator::None, vec![]);
//     for i in 0..tokens.len() {
//       if i > 0 && i < tokens.len() - 1 {
//           let token = tokens[i].clone();
//           if let Token::Operator(o) = token {
//               if version.1.clone().over(o.clone()) {
//                   println!("here");
//                   let tokens_copy = tokens.clone();
//                   let (mut a, mut b) = tokens_copy.split_at(i);
//                   let a = Vec::from(a);
//                   let b: Vec<Token> = Vec::from(&b[1..]);
//                   version = (a, o, b);
//               }
//           }
//       }
//     }
//
//     println!("version: {:?}", version);
//     let a =
//         if version.0.len() == 1 {
//             if let Some(Token::Paren(toks)) = version.0.get(0) {
//                 if toks.len() == 1 {
//                     Possible::token(toks.get(0).unwrap().clone())
//                 } else {
//                     three_composer(toks.clone())
//                 }
//             } else {
//                 Possible::token(version.0.get(0).unwrap().clone())
//             }
//         } else {
//             three_composer(version.0)
//         };
//     let b =
//         if version.2.len() == 1 {
//             if let Some(Token::Paren(toks)) = version.2.get(0) {
//                 three_composer(toks.clone())
//             } else {
//                 Possible::token(version.2.get(0).unwrap().clone())
//             }
//         } else {
//             three_composer(version.2)
//         };
//     Possible::expression(Expression::new(a, version.1, b))
// }

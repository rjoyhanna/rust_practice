#[test] fn it_works() {
   println!("\nTesting...");
   let blah: [InfixToken;7] = [InfixToken::Operand(2), InfixToken::Operator(Operator::Add),
   InfixToken::Operand(3), InfixToken::Operator(Operator::Mul), InfixToken::Operand(4), InfixToken::Operator(Operator::Div), InfixToken::Operand(5)];
   println!("Case 0| 2+3*4/5=?2 3 4 * 5 / +: {:?}", infix_to_postfix(&blah));
   println!("\n");
   println!("\n");
   let blah: [InfixToken;5] = [InfixToken::Operand(23), InfixToken::Operator(Operator::Add),
   InfixToken::Operand(4), InfixToken::Operator(Operator::Mul), InfixToken::Operand(2)];
   println!("Case 1| 23+4*2=?23 4 2*+: {:?}", infix_to_postfix(&blah));
   println!("\n");
   println!("\n");
   //2 + (5 - 2) * 3
   let blah: [InfixToken;9] = [InfixToken::Operand(2), InfixToken::Operator(Operator::Add),
   InfixToken::LeftParen, InfixToken::Operand(5), InfixToken::Operator(Operator::Sub), InfixToken::Operand(2), InfixToken::RightParen, InfixToken::Operator(Operator::Mul), InfixToken::Operand(3)];
   println!("Case 2| 2+(5-2)*3=? 2 5 2 - 3 * +: {:?}", infix_to_postfix(&blah));
   
   // let blah: [InfixToken;2] = [InfixToken::Operand(5), InfixToken::Operator(Operator::Sub)];
   // println!("{:?}", infix_to_postfix(&blah));
   // //(2+3
   // let blah: [InfixToken;4] = [InfixToken::LeftParen, InfixToken::Operand(2),
   // InfixToken::Operator(Operator::Mul),
   // InfixToken::Operand(3)];
   // println!("{:?}", infix_to_postfix(&blah));
   // // 2+3)
   // let blah: [InfixToken;5] = [InfixToken::Operand(2),
   // InfixToken::Operator(Operator::Mul),
   // InfixToken::Operand(3),
   // InfixToken::RightParen, InfixToken::RightParen];
   // println!("{:?}", infix_to_postfix(&blah));
   // // (2*)3
   // let blah: [InfixToken;5] = [InfixToken::LeftParen, InfixToken::Operand(2),
   // InfixToken::Operator(Operator::Mul),
   // InfixToken::RightParen, InfixToken::Operand(3)];
   // println!("{:?}", infix_to_postfix(&blah));
   // //1++2
   // let blah: [InfixToken;4] = [InfixToken::Operand(1),
   // InfixToken::Operator(Operator::Add),
   // InfixToken::Operator(Operator::Add),
   // InfixToken::Operand(2)];
   // println!("{:?}", infix_to_postfix(&blah));
   // //2(+3)
   // let blah: [InfixToken;4] = [InfixToken::Operand(2),
   // InfixToken::LeftParen,
   // InfixToken::Operator(Operator::Add),
   // InfixToken::Operand(3)];
   // println!("{:?}", infix_to_postfix(&blah));
   // let blah: [InfixToken;11] = [InfixToken::Operand(2),
   // InfixToken::Operator(Operator::Add), InfixToken::LeftParen, InfixToken::LeftParen,
   // InfixToken::Operand(3),
   // InfixToken::Operator(Operator::Sub),
   // InfixToken::Operand(1), InfixToken::RightParen, InfixToken::Operator(Operator::Mul),
   // InfixToken::Operand(4), InfixToken::RightParen];
   // println!("{:?}", infix_to_postfix(&blah));
   // let blah: [InfixToken;9] = [InfixToken::Operand(1),
   // InfixToken::Operator(Operator::Add), InfixToken::LeftParen, InfixToken::LeftParen,
   // InfixToken::Operand(2),
   // InfixToken::Operator(Operator::Add),
   // InfixToken::Operand(3), InfixToken::RightParen, InfixToken::RightParen];
   // println!("{:?}", infix_to_postfix(&blah));
   // let blah: [InfixToken;7] = [InfixToken::LeftParen, InfixToken::LeftParen,
   // InfixToken::Operand(3),
   // InfixToken::Operator(Operator::Sub),
   // InfixToken::Operand(1), InfixToken::RightParen, InfixToken::RightParen];
   // println!("{:?}", infix_to_postfix(&blah));
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
   Add,
   Sub,
   Mul,
   Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
   Operator(Operator),
   Operand(isize),
   LeftParen,
   RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
   Operator(Operator),
   Operand(isize),
}

//Conserves order by popping out all operators between left and right parentheses
fn empty_op_stack(op: &mut Vec<InfixToken>, out_exp: &mut Vec<PostfixToken>) -> bool {
   //check for empty stack
   let mut length = op.len();
   let mut last_token: InfixToken;
   match length {
      0 => {
         //println!("Marker 1");
         return false 
      },
      _ => last_token = op.pop().unwrap(),  
   }
   while last_token != InfixToken::LeftParen && length > 0 {
      //println!("last_token: {:?}", last_token);
      if let InfixToken::Operator(ref operation) = last_token {
         out_exp.push(PostfixToken::Operator(*operation));
      }
      else {
         //println!("Marker 2");
         return false;
      }
      length = op.len();
      match length {
         0 => {
            //println!("Marker 3");
            return false
         }, //if end of stack is reached without LeftParen return None
         _ => last_token = op.pop().unwrap(),
      }
   }

   //this section is only to pop the LeftParen if it runs all the way through
   //println!("op_stack: {:?}", op);
   return true;
}

fn precedence_loop(in_token: InfixToken, op: &mut Vec<InfixToken>, out_exp: &mut Vec<PostfixToken>) {
   //println!("op_stack 1: {:?}", op);
   match op.len() {
      0 => {op.push(in_token)},
      _ => {
         let last_token = op.pop().unwrap();
         match last_token {
            InfixToken::Operator(ref operation) => { //if operator exists on stack, check its precedence to new member
               match in_token { //changes behavior to accommodate precedence rules
                  InfixToken::Operator(Operator::Mul) | InfixToken::Operator(Operator::Div) => { //if current token is * or /, then only + and - have less precedence
                     match last_token {
                        InfixToken::Operator(Operator::Add) | InfixToken::Operator(Operator::Sub) => {
                           op.push(InfixToken::Operator(*operation));
                           op.push(in_token);
                        },
                        _=> {
                           out_exp.push(PostfixToken::Operator(*operation));
                           let stack = op;
                           let out = out_exp;
                           precedence_loop(in_token, stack, out);
                        },
                     }
                  },
                  InfixToken::Operator(ref operator) => {
                     out_exp.push(PostfixToken::Operator(*operation));
                     let stack = op;
                     let out = out_exp;
                     precedence_loop(InfixToken::Operator(*operator), stack, out);
                  },
                  _=> panic!("This option should never be reached!"),
               }
            },
            InfixToken::LeftParen => {
               op.push(last_token);
               op.push(in_token);
            }, //if LeftParen is on the stack, just push new value anyway
            _ => panic!("Should not be reached!"),
         }
      },
   }
}


/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`; 
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
   let mut post_fix_exp: Vec<PostfixToken> = Vec::new();
   let mut op_stack: Vec<InfixToken> = Vec::new();
   let mut index = 0;
   let last_index = tokens.len() - 1;
   let mut operator_count = 0;
   let mut operand_count = 0;
   let mut left_count = 0;
   let mut right_count = 0;

   //checks for immediately invalid inputs
   if tokens.len() == 0 {return Option::None};
   if let InfixToken::Operator(_) = tokens[0] { return Option::None}
   else if let InfixToken::RightParen = tokens[0] {return Option::None};
   if let InfixToken::Operator(_) = tokens[tokens.len()-1] {return Option::None}
   else if let InfixToken::LeftParen = tokens[tokens.len()-1] {return Option::None};

   //rearranges to postfix expression
   while index <= last_index {
      //println!("Token: {:?}", tokens[index]);
      match &tokens[index] {
         &InfixToken::Operand(ref number) => {
            match index{
               0 => {
                  post_fix_exp.push(PostfixToken::Operand(*number));
               },
               _ => {
                  match &tokens[index-1] {
                     &InfixToken::Operand(_) | &InfixToken::RightParen => return Option::None,
                     _ => {
                        post_fix_exp.push(PostfixToken::Operand(*number));

                     },
                  }
               },
            }
            operand_count += 1;
         },
         &InfixToken::LeftParen => {
            left_count += 1;
            match index{
               0 => {
                  op_stack.push(InfixToken::LeftParen);
               },
               _ => {
                  match &tokens[index-1] {
                     &InfixToken::Operand(_) | &InfixToken::RightParen => return Option::None,
                     _ => {
                        op_stack.push(InfixToken::LeftParen);
                     },
                  }
               },
            }
         },
         &InfixToken::RightParen => {
            right_count += 1;
            match index{
               0 => {
                  return Option::None;
               },
               _ => {
                  match &tokens[index-1] {
                     &InfixToken::Operator(_) | &InfixToken::LeftParen => {
                        //println!("Marker 4:");
                        return Option::None
                     },
                     _ => {
                        //println!("OP Length: {:?}", op_stack.len());
                        match empty_op_stack(&mut op_stack, &mut post_fix_exp) {
                           true => {
                              index +=1;
                              continue}
                              ,
                           false => {
                              //println!("Marker 5"); 
                              return Option::None
                           },
                        };
                     },
                  }
               },
            }
         },
         &InfixToken::Operator(ref operator) => {
            //println!("Opstack 2: {:?}", op_stack);
            match index{
               0 => {
                  return Option::None;
               },
               _ => {
                  match &tokens[index-1] {
                     &InfixToken::Operator(_) | &InfixToken::LeftParen => {
                        //println!("Marker 6:");
                        return Option::None
                     },
                     _ => {
                        precedence_loop(InfixToken::Operator(*operator), &mut op_stack, &mut post_fix_exp);
                        operator_count += 1;

                     },
                  }
               },
            }
         },
      }
      //println!("Token: {:?} |  Opstack: {:?} |  Output: {:?} ", tokens[index], op_stack, post_fix_exp);
      index += 1;
   }
   //pops remaining operators after reaching the end of the string
   while op_stack.len() > 0 {
      let last_token = op_stack.pop().unwrap();
      if let InfixToken::Operator(operation) = last_token {
         post_fix_exp.push(PostfixToken::Operator(operation));
      }
      else if let InfixToken::LeftParen = last_token {
         return Option::None;
      }
   }
   if operator_count != (operand_count-1) || left_count != right_count {
      return Option::None;
   };
   return Option::Some(post_fix_exp)
}
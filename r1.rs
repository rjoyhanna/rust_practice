#[test] fn testing_ground () {
   // let blah = [Token::Operand(1), Token::Operand(3),
   //     Token::Operand(-3), Token::Operator(Operator::Add),
   //     Token::Operator(Operator::Sub), Token::Operand(2),
   //      Token::Operator(Operator::Add), Token::Operator(Operator::Sub)];
   let blah = [Token::Operand(5), Token::Operand(4)];
   println!("Eval returned: {:?}", eval(&blah[..]));
}

#[derive(Debug)]
pub enum Operator {
   Add, 
   Sub,
   Mul,
}

#[derive(Debug)]
pub enum Token {
   Operator(Operator),
   Operand(isize),
}

fn operate (operation: &Operator, operands: &mut Vec<isize>) -> Option<isize> {
   let result: isize;
   if operands.len() < 2 { return Option::None}; //invalid input should break here
   match operation {
      &Operator::Add => result = operands.pop().unwrap() + operands.pop().unwrap(),
      &Operator::Sub => result = -(operands.pop().unwrap() - operands.pop().unwrap()),
      &Operator::Mul => result = operands.pop().unwrap() * operands.pop().unwrap(),
   }
   return Option::Some(result);
}

pub fn eval (tokens: &[Token]) -> Option<isize> {
   //Stores operands and performs operations when applicable
   let mut operators: Vec<&Operator> = Vec::new();
   let mut operands: Vec<isize> = Vec::new();
   if tokens.len() == 0 {return Option::None};
   for value in tokens.iter() {
      match value {
         &Token::Operator(ref a) => {operators.push(a);
            let result = operate(a, &mut operands);
            if result == Option::None {return Option::None};
            operands.push(result.unwrap()) },
         &Token::Operand(b) => operands.push(b),
      }
   }
   //checks to make sure all operands are operated on
   if operands.len() != 1 {return Option::None};
   
   //Returns if successful
   Option::Some(operands.pop().unwrap())
}
use std::process::Output;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  INTEGER,
  PLUS,
  MINUS,
  STAR,
  SLASH,
  LPAREN,
  RPAREN,
  SEMI,
}

pub fn lexer(input: &str) -> String {

  let mut line_counter: u32 = 1;
  let mut column_counter: u32 = 1;

  let mut output = String::from("[\n");
  let mut chars = input.chars().peekable();
  while let Some(char) = chars.next() {
    line_counter+=1;
    if char == '\n' {
      line_counter = 1;
      column_counter += 1;
    }
    output.push_str(format!("{{ \"type\": \"{:?}\", \"literal\": \"{}\", \"line\": {}, \"column\": {} }},\n",
      match char.is_digit(10) {
        true => {
          let mut _num = String::from(char);
          while let Some(&next_char) = chars.peek() {
             if next_char.is_digit(10) {
                        _num.push(chars.next().unwrap());
                    } else {
                        break;
                    }
          }
          Token::INTEGER
        },
        false =>  match char {
          '+' => Token::PLUS,
          '-' => Token::MINUS,
          '*' => Token::STAR,
          '/' => Token::SLASH,
          '(' => Token::LPAREN,
          ')' => Token::RPAREN,
          ';' => Token::SEMI,
          _ => continue
        },
      },
      match char.is_digit(10) {
        true => {
          let mut num = String::from(char);
          while let Some(&next_char) = chars.peek() {
             if next_char.is_digit(10) {
                        num.push(chars.next().unwrap());
                    } else {
                        break;
                    }
          }
          num
        },
        false => char.to_string(),
      },
      column_counter,
      line_counter,
    ).as_str());
  }
  output.push_str("]");
  println!("{output}");
  return output;
}

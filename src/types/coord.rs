use tokenizer::{PeekableTokens, Token};


pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub m: Option<f64>,
}

impl Coord {
    pub fn from_tokens(tokens: &mut PeekableTokens) ->  Result<Self, &'static str> {
        let x = match tokens.next() {
            Some(Token::Number(n)) => n,
            _ => return Err("Expected a number for the X coordinate"),
        };
        let y = match tokens.next() {
            Some(Token::Number(n)) => n,
            _ => return Err("Expected a number for the Y coordinate"),
        };
        Ok(Coord {x: x, y: y, z: None, m: None})
    }
}
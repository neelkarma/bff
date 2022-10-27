pub enum Token {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

impl Token {
    pub fn some_from(chr: char) -> Option<Self> {
        match chr {
            '>' => Some(Self::Right),
            '<' => Some(Self::Left),
            '+' => Some(Self::Increment),
            '-' => Some(Self::Decrement),
            '.' => Some(Self::Output),
            ',' => Some(Self::Input),
            '[' => Some(Self::LoopStart),
            ']' => Some(Self::LoopEnd),
            _ => None,
        }
    }
}

pub fn parse(source: &str) -> Vec<Token> {
    source
        .chars()
        .map(Token::some_from)
        .filter_map(|token| token)
        .collect()
}

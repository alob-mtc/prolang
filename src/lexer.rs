struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> &'static mut Lexer {
        let l = Lexer {
            input,
            position: todo!(),
            read_position: todo!(),
            ch: todo!(),
        };

        return &mut l;
    }

    pub fn read_char() {}
}

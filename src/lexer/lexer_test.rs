use super::super::token::token::{self, Token};
use crate::lexer::Lexer;

#[cfg(test)]

struct Expect {
    expected_type: token::TokenType,
    expected_literal: String,
}
#[test]
fn test_next_token() {
    let input = "let five = 5 \
    ;";

    let tests: Vec<Expect> = vec![
        Expect {
            expected_type: token::LET.to_string(),
            expected_literal: "let".to_string(),
        },
        Expect {
            expected_type: token::IDENT.to_string(),
            expected_literal: "five".to_string(),
        },
        Expect {
            expected_type: token::ASSIGN.to_string(),
            expected_literal: "=".to_string(),
        },
        Expect {
            expected_type: token::INT.to_string(),
            expected_literal: "1".to_string(),
        },
        Expect {
            expected_type: token::SEMICOLON.to_string(),
            expected_literal: ";".to_string(),
        },
        Expect {
            expected_type: token::EOF.to_string(),
            expected_literal: "".to_string(),
        },
    ];

    let mut l = Lexer::new(String::from(input));

    for tt in tests.iter() {
        let tok = l.next_token();

        println!("my token {} - {}", tok.literal, tok.token);
        if tok.token != tt.expected_type {}

        assert_eq!(tok.token, tt.expected_type, "failed");

        // if tok.token !=
    }
}

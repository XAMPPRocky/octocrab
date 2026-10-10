#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    Comment,
    Attribute,
    Keyword,
    Type,
    Macro,
    StringLiteral,
    NumberLiteral,
    Function,
    Operator,
    Punctuation,
    Key,
    Identifier,
    Whitespace,
}

impl TokenType {
    pub fn class(&self) -> &'static str {
        match self {
            TokenType::Comment => "text-gray-500 italic",
            TokenType::Attribute => "text-amber-400/90",
            TokenType::Keyword => "text-purple-400 font-semibold",
            TokenType::Type => "text-cyan-300 font-medium",
            TokenType::Macro => "text-blue-400 font-semibold",
            TokenType::StringLiteral => "text-emerald-400",
            TokenType::NumberLiteral => "text-amber-300 font-mono",
            TokenType::Function => "text-sky-300",
            TokenType::Operator => "text-pink-400",
            TokenType::Punctuation => "text-gray-400",
            TokenType::Key => "text-sky-300 font-medium",
            TokenType::Identifier => "text-gray-200",
            TokenType::Whitespace => "",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
}

/// Tokenizes Rust source code into lines of styled tokens
pub fn highlight_rust(code: &str) -> Vec<Vec<Token>> {
    let mut lines = Vec::new();

    for raw_line in code.lines() {
        let mut tokens = Vec::new();
        let chars: Vec<char> = raw_line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            // 1. Whitespace
            if chars[i].is_whitespace() {
                let start = i;
                while i < len && chars[i].is_whitespace() {
                    i += 1;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::Whitespace,
                });
                continue;
            }

            // 2. Line comment
            if chars[i] == '/' && i + 1 < len && chars[i + 1] == '/' {
                tokens.push(Token {
                    text: chars[i..len].iter().collect(),
                    token_type: TokenType::Comment,
                });
                break;
            }

            // 3. Attribute (#[...])
            if chars[i] == '#' && i + 1 < len && chars[i + 1] == '[' {
                let start = i;
                while i < len && chars[i] != ']' {
                    i += 1;
                }
                if i < len && chars[i] == ']' {
                    i += 1;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::Attribute,
                });
                continue;
            }

            // 4. String literal
            if chars[i] == '"' {
                let start = i;
                i += 1;
                let mut escaped = false;
                while i < len {
                    if escaped {
                        escaped = false;
                    } else if chars[i] == '\\' {
                        escaped = true;
                    } else if chars[i] == '"' {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::StringLiteral,
                });
                continue;
            }

            // 5. Numeric literal
            if chars[i].is_ascii_digit() {
                let start = i;
                while i < len
                    && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_')
                {
                    i += 1;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::NumberLiteral,
                });
                continue;
            }

            // 6. Word: Identifier, Keyword, Type, Macro, or Function
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                // Check if macro (followed by '!')
                if i < len && chars[i] == '!' && (i + 1 == len || chars[i + 1] != '=') {
                    i += 1;
                    let macro_name: String = chars[start..i].iter().collect();
                    tokens.push(Token {
                        text: macro_name,
                        token_type: TokenType::Macro,
                    });
                    continue;
                }

                // Check if function call (followed by '(' ignoring spaces)
                let mut next_non_ws = i;
                while next_non_ws < len && chars[next_non_ws].is_whitespace() {
                    next_non_ws += 1;
                }
                let is_func_call = next_non_ws < len && chars[next_non_ws] == '(';

                let token_type = match word.as_str() {
                    "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn"
                    | "else" | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl"
                    | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref"
                    | "return" | "self" | "Self" | "static" | "struct" | "super" | "trait"
                    | "true" | "type" | "unsafe" | "use" | "where" | "while" => TokenType::Keyword,
                    "Result" | "Option" | "Some" | "None" | "Ok" | "Err" | "Octocrab" | "Vec"
                    | "String" | "Box" | "Error" | "u64" | "u32" | "i64" | "i32" | "usize"
                    | "bool" | "State" | "ParentContext" | "QueryNode" | "HashMap" | "Value"
                    | "Map" => TokenType::Type,
                    w if w.starts_with(|c: char| c.is_uppercase()) => TokenType::Type,
                    _ if is_func_call => TokenType::Function,
                    _ => TokenType::Identifier,
                };

                tokens.push(Token {
                    text: word,
                    token_type,
                });
                continue;
            }

            // 7. Multi-character punctuation/operators
            if i + 1 < len {
                let two: String = chars[i..=i + 1].iter().collect();
                if matches!(
                    two.as_str(),
                    "::" | "->" | "=>" | "==" | "!=" | "<=" | ">=" | "&&" | "||"
                ) {
                    tokens.push(Token {
                        text: two,
                        token_type: TokenType::Operator,
                    });
                    i += 2;
                    continue;
                }
            }

            // 8. Single character operators & punctuation
            let c = chars[i];
            let token_type = match c {
                '?' | '&' | '|' | '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '^' => {
                    TokenType::Operator
                }
                ';' | ',' | ':' | '.' | '(' | ')' | '{' | '}' | '[' | ']' => TokenType::Punctuation,
                _ => TokenType::Identifier,
            };

            tokens.push(Token {
                text: c.to_string(),
                token_type,
            });
            i += 1;
        }

        lines.push(tokens);
    }

    if lines.is_empty() {
        lines.push(Vec::new());
    }

    lines
}

/// Tokenizes JSON string into lines of styled tokens
pub fn highlight_json(json_str: &str) -> Vec<Vec<Token>> {
    let mut lines = Vec::new();

    for raw_line in json_str.lines() {
        let mut tokens = Vec::new();
        let chars: Vec<char> = raw_line.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            // Whitespace
            if chars[i].is_whitespace() {
                let start = i;
                while i < len && chars[i].is_whitespace() {
                    i += 1;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::Whitespace,
                });
                continue;
            }

            // String
            if chars[i] == '"' {
                let start = i;
                i += 1;
                let mut escaped = false;
                while i < len {
                    if escaped {
                        escaped = false;
                    } else if chars[i] == '\\' {
                        escaped = true;
                    } else if chars[i] == '"' {
                        i += 1;
                        break;
                    }
                    i += 1;
                }

                // Check if followed by ':' (ignoring whitespace) -> Key
                let mut lookahead = i;
                while lookahead < len && chars[lookahead].is_whitespace() {
                    lookahead += 1;
                }
                let is_key = lookahead < len && chars[lookahead] == ':';

                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: if is_key {
                        TokenType::Key
                    } else {
                        TokenType::StringLiteral
                    },
                });
                continue;
            }

            // Numbers
            if chars[i].is_ascii_digit()
                || (chars[i] == '-' && i + 1 < len && chars[i + 1].is_ascii_digit())
            {
                let start = i;
                i += 1;
                while i < len
                    && (chars[i].is_ascii_digit()
                        || chars[i] == '.'
                        || chars[i] == 'e'
                        || chars[i] == 'E'
                        || chars[i] == '+'
                        || chars[i] == '-')
                {
                    i += 1;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::NumberLiteral,
                });
                continue;
            }

            // Keywords: true, false, null
            if chars[i].is_alphabetic() {
                let start = i;
                while i < len && chars[i].is_alphabetic() {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                tokens.push(Token {
                    text: word,
                    token_type: TokenType::Keyword,
                });
                continue;
            }

            // Punctuation
            tokens.push(Token {
                text: chars[i].to_string(),
                token_type: TokenType::Punctuation,
            });
            i += 1;
        }

        lines.push(tokens);
    }

    if lines.is_empty() {
        lines.push(Vec::new());
    }

    lines
}

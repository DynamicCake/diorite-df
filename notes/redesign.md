Parser struct now an error list of lexer errors
Lexer errors are now parse errors ish
EOF errors should not be parse errors because it can mess with recovery

EOF error can just get passed down as there only is one

fn next() -> Spanned<Token<'_>> where lexer error is Token::Invalid and pushed to error list
EOF can only appear once meaning that `Vec<UnexpectedEOF<'_>>` is not allowed


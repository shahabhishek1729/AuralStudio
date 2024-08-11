#[cfg(test)]
mod tests {
    use crate::scanner::rtl_token::RTLToken;
    use crate::scanner::scanner::*;

    #[test]
    fn test_valid_strings() {
        let source = "string hello world done";
        let mut scanner = Scanner::new(source);

        match scanner.scan() {
            Ok(_) => assert!(true),
            Err(msg) => assert!(false, "{}", msg),
        }
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].rtl_token, RTLToken::StringVal);
        assert_eq!(
            scanner.tokens[0].literal,
            Some(Literal::RTLString("hello world".to_string()))
        );
        assert_eq!(scanner.tokens[1].rtl_token, RTLToken::EOF);
    }

    #[test]
    fn test_valid_strings_symbols() {
        let source = "string hello world exclamation sign done";
        let mut scanner = Scanner::new(source);

        match scanner.scan() {
            Ok(_) => assert!(true),
            Err(msg) => assert!(false, "{}", msg),
        }
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].rtl_token, RTLToken::StringVal);
        assert_eq!(
            scanner.tokens[0].literal,
            Some(Literal::RTLString("hello world!".to_string()))
        );
        assert_eq!(scanner.tokens[1].rtl_token, RTLToken::EOF);
    }

    #[test]
    fn test_invalid_strings() {
        let source = "string hello world";
        let mut scanner = Scanner::new(source);

        match scanner.scan() {
            Ok(_) => assert!(false, "Should have failed"),
            Err(_) => assert!(true),
        }
    }

    // #[test]
    // fn test_valid_numerics() {
    //     let source = "numeric ten thousand four hundred and sixty two over";
    //     let mut scanner = Scanner::new(source);
    //
    //     match scanner.scan() {
    //         Ok(_) => assert!(true),
    //         Err(msg) => assert!(false, "{}", msg),
    //     }
    //     assert_eq!(scanner.tokens.len(), 2);
    //     assert_eq!(scanner.tokens[0].rtl_token, RTLToken::NumericVal);
    //     assert_eq!(
    //         scanner.tokens[0].literal,
    //         Some(Literal::RTLNumeric(10462f64))
    //     );
    //     assert_eq!(scanner.tokens[1].rtl_token, RTLToken::EOF);
    // }
    //
    // #[test]
    // fn test_invalid_numerics() {
    //     let source = "numeric hello world over";
    //     let mut scanner = Scanner::new(source);
    //
    //     match scanner.scan() {
    //         Ok(_) => assert!(false, "Should have failed"),
    //         Err(_) => assert!(true),
    //     }
    // }

    // #[test]
    // fn test_float_numerics() {
    //     let source = "numeric ten thousand four hundred and sixty two point four three over";
    //     let mut scanner = Scanner::new(source);
    //
    //     match scanner.scan() {
    //         Ok(_) => assert!(true),
    //         Err(msg) => assert!(false, "{}", msg),
    //     }
    //     assert_eq!(scanner.tokens.len(), 2);
    //     assert_eq!(scanner.tokens[0].rtl_token, RTLToken::NumericVal);
    //     assert_eq!(
    //         scanner.tokens[0].literal,
    //         Some(Literal::RTLNumeric(10462.43f64))
    //     );
    //     assert_eq!(scanner.tokens[1].rtl_token, RTLToken::EOF);
    // }
    //
    #[test]
    fn test_identifiers_basic() {
        let source = "here_is_var1";
        let mut scanner = Scanner::new(source);

        match scanner.scan() {
            Ok(_) => assert!(true),
            Err(msg) => assert!(false, "{}", msg),
        }
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].rtl_token, RTLToken::ObjIdentifier);
        assert_eq!(scanner.tokens[1].rtl_token, RTLToken::EOF);
    }

    #[test]
    fn test_identifiers_invalid() {
        let source = "0_here_is_var1";
        let mut scanner = Scanner::new(source);

        let s = scanner.scan();
        match s {
            Ok(_) => assert!(false, "Should have failed"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_variable_declaration() {
        let source = "let josh_how_are_you be string i'm great how are you? done";
        let mut scanner = Scanner::new(source);

        match scanner.scan() {
            Ok(_) => assert!(true),
            Err(msg) => assert!(false, "{}", msg),
        }

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].rtl_token, RTLToken::VarIdentifier);
        assert_eq!(scanner.tokens[1].rtl_token, RTLToken::ObjIdentifier);
        assert_eq!(
            scanner.tokens[1].literal,
            Some(Literal::RTLIdentifier("josh_how_are_you".to_string()))
        );
        assert_eq!(scanner.tokens[2].rtl_token, RTLToken::AssnEq);
        assert_eq!(scanner.tokens[3].rtl_token, RTLToken::StringVal);
        assert_eq!(
            scanner.tokens[3].literal,
            Some(Literal::RTLString("i'm great how are you?".to_string()))
        );
        assert_eq!(scanner.tokens[4].rtl_token, RTLToken::EOF);
    }

    #[test]
    fn test_functions() {
        let source = "define sum of lst\nlet l be len of lst done\nif l equals 0\noutput string your list was empty done";
        let mut scanner = Scanner::new(source);

        match scanner.scan() {
            Ok(_) => assert!(true),
            Err(msg) => assert!(false, "{}", msg),
        }

        dbg!(&scanner.tokens);
        assert_eq!(scanner.tokens.len(), 20);
        assert_eq!(scanner.tokens[0].rtl_token, RTLToken::FunctionIdentifier);
        assert_eq!(scanner.tokens[1].rtl_token, RTLToken::ObjIdentifier);
        assert_eq!(
            scanner.tokens[1].literal,
            Some(Literal::RTLIdentifier("sum".to_string()))
        );
        assert_eq!(scanner.tokens[2].rtl_token, RTLToken::ObjIdentifier);
        assert_eq!(
            scanner.tokens[2].literal,
            Some(Literal::RTLIdentifier("lst".to_string()))
        );
        // assert_eq!(scanner.tokens[3].rtl_token, RTLToken::BlockStart);
        assert_eq!(scanner.tokens[3].rtl_token, RTLToken::LineBreak);
        assert_eq!(scanner.tokens[4].rtl_token, RTLToken::VarIdentifier);
        assert_eq!(scanner.tokens[5].rtl_token, RTLToken::ObjIdentifier);
        assert_eq!(
            scanner.tokens[5].literal,
            Some(Literal::RTLIdentifier("l".to_string()))
        );
        assert_eq!(scanner.tokens[6].rtl_token, RTLToken::AssnEq);
        assert_eq!(scanner.tokens[7].rtl_token, RTLToken::FnCallIdentifier);
        assert_eq!(scanner.tokens[8].rtl_token, RTLToken::ObjIdentifier);
        assert_eq!(
            scanner.tokens[8].literal,
            Some(Literal::RTLIdentifier("len".to_string()))
        );
        assert_eq!(scanner.tokens[9].rtl_token, RTLToken::ObjIdentifier);
        assert_eq!(
            scanner.tokens[9].literal,
            Some(Literal::RTLIdentifier("lst".to_string()))
        );
        assert_eq!(scanner.tokens[10].rtl_token, RTLToken::ExprEnd);
        assert_eq!(scanner.tokens[11].rtl_token, RTLToken::LineBreak);
        assert_eq!(scanner.tokens[12].rtl_token, RTLToken::IfIdentifier);
        assert_eq!(scanner.tokens[13].rtl_token, RTLToken::ObjIdentifier);
        assert_eq!(scanner.tokens[14].rtl_token, RTLToken::EqComparator);
        assert_eq!(scanner.tokens[15].rtl_token, RTLToken::NumericVal);
        assert_eq!(scanner.tokens[15].literal, Some(Literal::RTLNumeric(0f64)));
        assert_eq!(scanner.tokens[16].rtl_token, RTLToken::LineBreak);
        assert_eq!(scanner.tokens[17].rtl_token, RTLToken::PrintToken);
        assert_eq!(scanner.tokens[18].rtl_token, RTLToken::StringVal);
        assert_eq!(
            scanner.tokens[18].literal,
            Some(Literal::RTLString("your list was empty".to_string()))
        );
        assert_eq!(scanner.tokens[19].rtl_token, RTLToken::EOF);
    }

    // #[test]
    // fn test_file() {
    //     let source = "function snake sum list over list snake list two over open\nvariable l equals call len list over\nif l equal to numeric zero over open\nstring your list was empty over";
    //
    //     let mut scanner = Scanner::new(source);
    //     scanner.scan();
    //     dbg!(scanner.tokens);
    //
    //     let mut decompiler = Decompiler::new(source).unwrap();
    //     let c = decompiler.decompile();
    //     match c {
    //         Ok(_) => {}
    //         Err(_) => {}
    //     };
    //
    //     assert!(false);
    // }
    //
    // use crate::decompiler::Decompiler;
    // #[test]
    // fn full() {
    //     let source = "variable x equals numeric three over\nif x modulo numeric to over equal to numeric zero over open\ncall print string even over over\nif over\nelse open\ncall print string odd over over\nelse over";
    //     let mut decompiler = Decompiler::new(source).unwrap();
    //     decompiler.decompile().unwrap();
    //     dbg!("PREPARE");
    //     dbg!(decompiler.py);
    //     assert!(false);
    // }
}

mod dfnum {
    use crate::common::ast::DfNumber;

    fn test_num(txt: &str, exp: &str, val: i64) {
        let num: DfNumber = txt.try_into().unwrap();
        assert_eq!(val, num.value());
        assert_eq!(format!("\"{exp}\""), serde_json::to_string(&num).unwrap());
    }

    #[test]
    fn parse_single() {
        test_num("1", "1.000", 1000);
        test_num("-1", "-1.000", -1000);
    }

    #[test]
    fn parse_normal() {
        test_num("-420", "-420.000", -420000);
        test_num("420", "420.000", 420000);
        test_num("-123456789", "-123456789.000", -123456789000);
        test_num("123456789", "123456789.000", 123456789000);
        test_num("0", "0.000", 0);
    }

    #[test]
    fn parse_decimal() {
        test_num("1.000", "1.000", 1000);
        test_num("-1.000", "-1.000", -1000);

        test_num(".123", "0.123", 123);
        test_num("-.123", "-0.123", -123);

        test_num("1.123", "1.123", 1123);
        test_num("-1.123", "-1.123", -1123);

        test_num(".3", "0.300", 300);
        test_num("-.3", "-0.300", -300);

        test_num("0.12", "0.120", 120);
        test_num("1.1", "1.100", 1100);

        test_num(".", "0.000", 0); // This should never happen because of lexer, but this is a reasonable responce
        test_num("-.", "0.000", 0); // -. is allowed, but should never happen
    }

    #[test]
    fn parse_higher() {
        test_num(
            "-9223372036854775.807",
            "-9223372036854775.807",
            -9223372036854775807,
        );
        test_num("-9223372036854775.808", "-9223372036854775.808", i64::MIN);

        test_num(
            "9223372036854775.807",
            "9223372036854775.807",
            9223372036854775807,
        );
        test_num("9223372036854775.807", "9223372036854775.807", i64::MAX);
    }

    #[test]
    fn parse_invalid() {
        use crate::common::ast::DfNumberParseError;
        assert!(matches!(
            DfNumber::try_from("").unwrap_err(),
            DfNumberParseError::EmptyInput
        ));

        assert!(matches!(
            DfNumber::try_from("deezium").unwrap_err(),
            DfNumberParseError::UnexpectedChar
        ));

        assert!(matches!(
            DfNumber::try_from("1234567890987654321234567890").unwrap_err(),
            DfNumberParseError::TooBig
        ));

        assert!(matches!(
            DfNumber::try_from("1.123456789098765432").unwrap_err(),
            DfNumberParseError::TooPercise
        ));
    }
}

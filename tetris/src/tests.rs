// mod badinput00 {
//     use super::*;

//     #[test]
//     fn parse() -> Result<(), String> {
//         parse_test("badinput00", parse_res())
//     }
//     fn parse_res() -> Result<(), FarmParseError> {
//         Err(FarmParseError(String::from("Missing UFO (`U`)")))
//     }
// } How tests work in rust, use a mod for a single test.
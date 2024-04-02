fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = std::env::args().nth(1).ok_or("usage: ./cmd <input>")?;
    let input = std::fs::read_to_string(&file_name)?;
    let input: Box<[char]> = input.chars().collect();

    use lcalc3::scanner::Scanner;
    let output = Scanner::with_input(&input).scan_input()?;
    // println!("{output:#?}");

    use lcalc3::parser::Parser;
    let output = Parser::parse_lexemes(output)?;

    use lcalc3::eval::Scope;
    let output = Scope::create_global_scope().eval(output)?;

    println!("{output}");

    Ok(())
}

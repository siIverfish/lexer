


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "rand()";
    let input: Box<[char]> = input.chars().collect();

    use lcalc3::scanner::scan;
    let output = scan(&input)?;

    use lcalc3::parser::Parser;
    let output = Parser::parse(output)?;

    use lcalc3::eval::Scope;
    let output = Scope::create_global_scope().eval(output)?;

    println!("{output:#?}");

    Ok(())
}




fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "(1 + 345) * 4";
    let input: Box<[char]> = input.chars().collect();
    // use lcalc3::input::Input;
    // let input = Input::new(input);

    use lcalc3::scanner::scan;
    let output = scan(&input)?;

    println!("{output:#?}");

    Ok(())
}

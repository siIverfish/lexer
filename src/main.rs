


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "(1 + 3) * 4";
    let input = input.chars().collect::<Box<[char]>>();
    use lcalc3::scanner::scan;
    let output = scan(&*input)?;

    println!("{output:#?}");

    Ok(())
}

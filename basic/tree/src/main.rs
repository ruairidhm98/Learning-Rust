
fn main()
{
  let mut ns = Vec::new();
  ns.push("hello"); ns.push("world");
  for n in ns
  {
    print!("{} ", n);
  }
  println!();

  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("idiot");
  let trimmed = input.trim();
  match trimmed.parse::<usize>()
  {
    Ok(i) => println!("your integer input: {}", i),
    Err(..) => println!("you idiot"),
  }
}


fn area(w : i64, h : i64) -> i64
{
  return  w * h;
}

fn main()
{
  let x : i32 = 4;
  let y : i32 = 4;
  println!("{}", x + y);
  println!("{}", area(4, 5));
  let b : u128 = u128::pow(2, 65);
  println!("{}", b);
  let string = "hello".to_string();
  println!("{}", string);
}

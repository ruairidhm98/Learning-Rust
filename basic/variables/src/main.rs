
fn area(w : i64, h : i64) -> i64
{
  return  w * h;
}

fn print_vec(vec : &Vec<i32>)
{
  for n in vec.iter()
  {
    println!("{}", n);
  }
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
  
  let mut vec = vec![];
  vec.push(5);
  vec.push(10);

  print_vec(&vec);

  println!();

  let mut b_new = Box::new(5);
  *b_new = 2;
  *b_new += 1;
  println!("{} lives at {}", *b_new, &b_new);

  let s1 = String::from("hello");
  let _s2 = s1.clone();

  println!("{}, world!", s1);
}

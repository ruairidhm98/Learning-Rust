

#[derive(Debug)]
struct Person<'a> {
  name : &'a str,
  role : &'a str
}

fn print_employees(employees: &Vec<Person>) {
  for e in employees {
    println!("{:?}", e);
  }
}

fn smallest(v: &[i32]) -> &i32 {
  let mut s = &v[0];
  for r in &v[1..] {
    if *r < *s {
      s = r;
    }
  }
  s
}

fn main(){
  // Task 1
  let mut v = Vec::new();
  v.push(Person{name : "Alice", role : "Manager"});
  v.push(Person{name : "Bob" , role : "Sales"});
  v.push(Person{name : "Carol", role : "Programmer"});
  print_employees(&v);
  println!("v.len() = {}", v.len());
  
  // Task 2
  let n = [12, 42, 6, 8, 15, 24];
  let s = smallest(&n);
  println!("{}", s);

  // Task 3

  // Task 4
  let vec = vec![4, 8, 19, 27, 34, 10];
  {
    let r = &vec;
    println!("{}", r[0]);
  }
  let aside = vec;
  println!("{}", aside[0]);
}
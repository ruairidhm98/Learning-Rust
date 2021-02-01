
struct Node
{
  val  : i32,
  next : Option<Box<Node> >,
}

struct LinkedList
{
  head : Option<Box<Node> >,
  n_nodes : i32,
}

impl LinkedList
{
  // Recursively iterates through the list and prints the contents
  fn print_list(&self, head : &Option<Box<Node> >)
  {
    match head 
    {
      None => println!("NULL"),
      Some(node) =>
      {
        print!("{}->", node.val);
        self.print_list(&node.next);
      },
    } 
  }

  // Print the contents of the list
  pub fn print(&self)
  {
    self.print_list(&self.head);
  }

  // Return the size of the list
  pub fn size(&self) -> i32
  {
    self.n_nodes
  }
}

fn main()
{
  let ll = LinkedList
  { head : Some(Box::new(Node
    { 
      val : 1,
      next : Some(Box::new(Node
      { 
        val : 2,
        next : None
      }))
    })),
    n_nodes : 2
  };
  ll.print();
  println!("{}", ll.size());
}

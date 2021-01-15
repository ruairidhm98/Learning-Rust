
use std::fmt::Display;

struct Node<T : Display>
{
  m_data : T,
  m_next : Option<Box<Node<T> > >,
}

struct LinkedList<T : Display>
{
  m_head : Option<Box<Node<T> > >,
  m_size : i64, 
}

impl<T : Display> LinkedList<T>
{
  pub fn new_list(data : T) -> LinkedList<T>
  {
    let head = Some(Box::new(Node{ m_data : data, m_next : None }));
    let ll = LinkedList{ m_head : head, m_size : 1 };
    return ll;
  }

  pub fn new_append(data : T, connect : LinkedList<T>) -> LinkedList<T>
  {
    let size = connect.m_size + 1;
    let head = Some(Box::new(Node{ m_data : data, m_next : connect.m_head }));
    return LinkedList{ m_head : head, m_size : size };
  }

  fn print_list(&self, next : &Option<Box<Node<T> > >)
  {
    match next
    {
      None => println!("NULL"),
      Some(node) =>
      {
        print!("{}->", node.m_data);
        self.print_list(&node.m_next);
      }
    }
  }

  fn insert_head(self, data : T) -> LinkedList<T>
  {
    return LinkedList::<T>::new_append(data, self);
  }

  fn print(&self)
  {
    self.print_list(&self.m_head);
  }
}

fn main()
{

  let ll = LinkedList::<i32>::new_list(6);
  let new_ll = ll.insert_head(5);
  new_ll.print();

  let other_ll = LinkedList::<i64>::new_list(6);
  other_ll.print();

  let connect_ll = LinkedList::<i32>::new_append(4, new_ll);
  connect_ll.print();

}

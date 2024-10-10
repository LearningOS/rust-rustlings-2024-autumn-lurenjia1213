pub enum List {
    test(u32,Box<List>),
    Nil,
    test2(i32,Box<List>)
}
fn create_shit()->List{
    List::test(1,Box::new(List::test2(2,Box::new(List::Nil))))
}
fn main(){
    
   
    
}
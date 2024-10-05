// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

//

struct Book<'a,'b> {
    author: &'a str,
    title: &'b str,
}//配置生命周期参数，与结构体共存亡，（避免空指针的问题

fn main() {
    let name = String::from("Jill Smith");
    {
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };
    println!("{} by {}", book.title, book.author);
    }
    
}

fn main(){
    let mut x=Some(5);
    while let Some(y)=x{
        println!("{}",y);
        if y==0{x=None}
        else {x=Some(y-1)}
    }
}
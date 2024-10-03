// variables1.rs
//
// Make me compile!
//
// Execute `rustlings hint variables1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let x = 5;
    println!("x has the value {}", x);
    let arr:[[i32;2];2]=[[1,2],[3,4]];
    for i in 0..2{
        for j in 0..2{
        println!("{}",arr[i][j]);}
    }
    let mut i:u32=0;
    let wtf =loop{
        i+=1;
        if i>10{
            
            
            break 114514}
        println!("{}",i);
    };
println!("{wtf}");


    let mut opt=Some(0);
    while let Some(x)=opt{
        if x>40{
            opt=None;
            break;
        }
        opt=Some(x+1);
        println!("{}",x);
    }

    for i in 0..=10{
        println!("{i}");
    }





}

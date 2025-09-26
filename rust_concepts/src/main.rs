use std::io;

fn main(){
    let arr=[45,15,24,89,66];
    println!("enter index");
    let  mut index =String::new();
    io::stdin()
    .read_line(&mut index)
    .expect("incorrect");

    let index :usize = index
    .trim()
    .parse()
    .expect("wrong index");

    let element = arr[index];
    println!("the element at {} index is {}",index,element);
}
// function untuk memecah kata

fn split_strinng(s: String,delimiter: char,field:usize) -> String
{
   let parts : Vec<&str> = s.split(delimiter).collect();
   let result = parts.get(field);
   result.expect("msg").to_string()
}
fn main() {
    let chunk = split_strinng("hello,world".to_string(),',' ,1);
    println!("split string:{}",chunk);
}
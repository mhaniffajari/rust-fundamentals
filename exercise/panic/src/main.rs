// using panic for negative number in vector
fn loop_and_panic(numbers:Vec<i32>) 
{
   for num in numbers{
      if num < 0 {
         panic!("Negative number{}",num)
      }
      println!("number{}",num)
   }
}

fn main (){
   loop_and_panic(vec![1,2,6,7,-3,-4])
}
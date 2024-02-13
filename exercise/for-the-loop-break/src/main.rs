fn main(){
    for i in 1..=120{
        if i % 3 == 0 {
            // skip third number
            continue
        }
        println!("i = {}",i);
        if i == 22{
            break
            //stop with the loop
        }
    }
}
fn main() {
    let mut height = 201;
    height = height - 20;
    let result = if height > 180 {
        "Tall"
    }else if height > 170 {
        "Avreage"}
    else {
        "Short"
    };
    println!("Height: {}", result);

    let mut health = if height > 180 {
        "Good"
}else{"Unknown"};
    println!("Health: {}", health);

    health = if height< 180 {"true"} else {"false"};
    println!("Health: {}", health);
}
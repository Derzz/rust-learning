fn main(){
    let mut height = String::new();
    println!("How large do you want your triangle to be?");
    std::io::stdin().read_line(&mut height).expect("Failed to read!");
    let height: i32 = height.trim().parse().expect("Failed to convert!");

    for i in (1..height + 1).step_by(2){
        for _ in 0..i{
            print!("*");
        }
        println!("");
    }
}
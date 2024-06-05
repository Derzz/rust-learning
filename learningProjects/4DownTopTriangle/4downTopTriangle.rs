fn main(){
    println!("How high do you want the triangle?");
    let mut height = String::new();
    std::io::stdin().read_line(&mut height).expect("Failed to read!");
    let height: i32 = height.trim().parse().expect("Failed to convert!");

    for i in 0..height{
        for _ in 0 .. i+1{
            print!("*");
        }
        println!();
    }
}


fn main(){
    println!("How high do you want the triangle?");
    let mut height = String::new();
    std::io::stdin().read_line(&mut height).expect("Failed to read!");
    let mut height: i32 = height.trim().parse().expect("Failed to convert height");
    while height > 0 {
        for _ in 0..height{
            print!("*");
        }
        println!();
        height -= 1;
    }
}
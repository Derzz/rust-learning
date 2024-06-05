fn main(){
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("Failed to read");
    let mut num: i32 = num.trim().parse().expect("Please type a number");
    let mut product:i32 = 1;
    loop{
        if num == 1{
            break;
        }
        product *= num;
        num -= 1;
    }

    println!("The product is {product}");
}
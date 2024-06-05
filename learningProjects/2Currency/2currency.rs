// Rates are based on June 3, 2024
// 1 CAD = 0.73 usd
// 1 USD = 1.36 CAD

fn main(){
    println!(" ==== CURRENCY CONVERTER ====");
    let mut convert = String::new();
    let mut usd_cad = true; // True for usd to cad, false for cad to usd
    println!("Enter C to convert CAD to USD or U to convert USD to CAD");
    loop {
        std::io::stdin().read_line(&mut convert).expect("Failed to read!");
        convert = convert.trim().to_string();
        if convert != "C" && convert != "U"{
            println!("Invalid response, please try again.");
            continue;
        }
        else if convert == "C"{
            usd_cad = false;
        }
        break;
    }

    println!("Input how much money you want to convert.");
    let mut money = String::new();
    std::io::stdin().read_line(&mut money).expect("Failed to read!");
    let money: i32 = money.trim().parse().expect("failed to convert");

    let converted_money = if usd_cad { money as f32 * 1.36 } else { money as f32 * 0.73 };
    println!("The converted money is {}", converted_money);

}
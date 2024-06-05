use rand::Rng;

fn function1(){
    println!("You got function 1!");
}

fn function2(){
    println!("You got function 2!")
}

fn function3(){
    println!("You got function 3!")
}

fn main(){
    let mut rng = rand::thread_rng();
    let num: u8 = rng.gen_range(0..3);

    match num{
        0=>function1(),
        1=>function2(),
        2=>function3(),
        _=>{},
    }
}

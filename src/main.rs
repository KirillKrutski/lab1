use std::io;
fn main() {
    let mut dote1 = String::new();

    io::stdin()
        .read_line(&mut dote1)
        .expect("Failed to read line");
    
let dote1: u32 = match dote1.trim().parse(){
    Ok(num) => num,
    Err(_) => continue,
};


    println!("Dote1 {dote1}");

    let mut dote2 = String::new();

    io::stdin()
        .read_line(&mut dote2)
        .expect("Failed to read line");

        
    let dote1: u32 = match dote1.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
    };

    println!("Dote2 {dote2}");



    if dote1 >= 2 && dote1 <= 5 && dote2 >= 2 && dote2 <=5 && dote2 - dote1 <= 0{
        println!("TRUE");
    } else  {
        println!("False");
    }
    
}   

mod encryptor;
use encryptor::encryptor::encrypt;

mod decryptor;
use decryptor::decryptor::decrypt;

fn main() {
    println!("Welcome to image encryptor/decryptor");

    loop {
        println!("What do you want to do :");
        println!("1. Encrypt");
        println!("2. Decrypt");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        choice.remove(choice.len()-1);

        if choice.len() == 1 {
            if choice.contains("1") {
                encrypt();
                break;
            }
            if choice.contains("2") {
                decrypt();
                break;
            }
        }

        println!("Wrong input: {:?}, choose 1 or 2", choice)
        
    }
}

extern crate copypasta;

use gnupg_rs::Key;
use gnupg_rs::GnuPG;

// use copypasta::{ClipboardContext, ClipboardProvider};

fn main() {
    // let mut ctx = ClipboardContext::new().unwrap();

    // let msg = "Hello, world!";
    // ctx.set_contents(msg.to_owned()).unwrap();

    // let content = ctx.get_contents().unwrap();

    // println!("{}", content);

    // Inputs
    let username = String::from("test");
    let email = String::from("test.amil").to_string();
    let password_value = "password";
    let password  = Some(password_value);
    let gnupg = GnuPG::new().unwrap();

    let key = Key::new(&username, &email, password, &gnupg).expect("msg");
    let exported_key = gnupg.export_key(&key);
    println!("Exported key:\n{}", exported_key);

    let exported_secret_key = gnupg.export_secret_key(&key).expect("msg");
    println!("Exported key:\n{}", exported_secret_key);

    let encrypted_msg = gnupg.encrypt(&key, "Hello, world!").unwrap();
    println!("Encrypted message:\n{}", encrypted_msg);

    let decrypted_msg = gnupg.decrypt(&key, &encrypted_msg).unwrap();
    println!("Decrypted message:\n{}\nDate: {}", decrypted_msg.text, decrypted_msg.date);

}
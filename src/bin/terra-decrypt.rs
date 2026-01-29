use std::path::Path;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        let program = &args[0];
        eprintln!("Usage: {program} <Encrypted.plr> [Decrypted.dplr]");
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let output_path = if args.len() >= 3 {
        Path::new(&args[2]).to_owned()
    } else {
        input_path.with_extension("dplr")
    };

    match terra_core::Player::decrypt_file(input_path, &output_path) {
        Ok(_) => println!(
            "Decryption successful! Output saved to '{}'",
            output_path.display()
        ),
        Err(e) => eprintln!("Decryption failed: {}", e),
    }
}

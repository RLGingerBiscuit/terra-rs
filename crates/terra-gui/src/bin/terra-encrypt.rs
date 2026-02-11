use std::path::Path;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        let program = &args[0];
        eprintln!("Usage: {program} <Decrypted.dplr> [Encrypted.plr]");
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let output_path = if args.len() >= 3 {
        Path::new(&args[2]).to_owned()
    } else {
        input_path.with_extension("plr")
    };

    let is_mobile = args.iter().any(|arg| arg == "--mobile" || arg == "-m");

    match terra_core::Player::encrypt_file(input_path, &output_path, is_mobile) {
        Ok(_) => println!(
            "Encryption successful! Output saved to '{}'",
            output_path.display()
        ),
        Err(e) => eprintln!("Encryption failed: {}", e),
    }
}

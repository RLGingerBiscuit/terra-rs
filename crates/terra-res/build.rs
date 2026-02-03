use std::{
    env,
    fs::File,
    io::{Result, Write},
};

fn main() -> Result<()> {
    println!("cargo:rerun-if-env-changed=PROFILE");
    let build_type = env::var("PROFILE").expect("Could not find 'PROFILE' environment variable.");

    File::create(env::current_dir().unwrap().join("build_type.txt"))?
        .write(build_type.as_bytes())?;

    Ok(())
}

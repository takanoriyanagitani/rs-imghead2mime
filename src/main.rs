use std::process::ExitCode;

use std::io;

use rs_imghead2mime::stdin2mime;

fn sub() -> Result<(), io::Error> {
    let mime: &str = stdin2mime()?;
    println!("{mime}");
    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}

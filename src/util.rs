pub fn exit_with_error(error: &str) -> ! {
    eprintln!("error: {}", error);
    std::process::exit(1);
}
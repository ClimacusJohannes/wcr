fn main() {
    if let Err(e) = wcr::cli().and_then(wcr::run) {
        eprintln!("{:#?}", e);
        std::process::exit(1);
    }
}

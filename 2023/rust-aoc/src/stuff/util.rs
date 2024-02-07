use std::env;
pub fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        panic!("not enough arguemnts");
    }
    let file_path = args[1].clone();
    return file_path;
}

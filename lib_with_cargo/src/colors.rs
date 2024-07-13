pub fn red(s: &str) -> String{
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn blue(s: &str) -> String{
    format!("\x1b[34m{}\x1b[0m", s)
}
// lambda entry point
fn handler() {
    println!("Hello, lambda!");
}

// local entry point
fn main() {
    handler();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler() {
        handler();
    }
}

fn main() {
    println!("Hello, world!");
    println!("More substantial change.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn random_test() {
        assert_eq!(2 + 2, 4);
    }
}

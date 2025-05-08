use palindrome::find_palindrome_product;

fn main() {
    match find_palindrome_product() {
        Some(num) => println!("Found {num}"),
        None => println!("No result found")
    }
}

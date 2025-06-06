use project_euler::problem_006;

fn main() {
    match problem_006::find_palindrome_product() {
        Some(num) => println!("Found {num}"),
        None => println!("No result found"),
    }
}

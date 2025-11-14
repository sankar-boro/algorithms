use algos::recursion_and_backtracking;

fn main() {
    let mut final_res: Vec<String> = Vec::new();
    recursion_and_backtracking::generate_parentheses(3, 0, 0, String::from(""), &mut final_res);
    println!("Final result: {final_res:?}")  
}
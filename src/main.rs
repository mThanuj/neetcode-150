mod problems;

fn main() {
    let result = problems::easy::valid_anagram::valid_anagram("bat".to_string(), "tab".to_string());
    println!("{result}");
}

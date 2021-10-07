fn main() {
    let pangram = "the quick brown fox jumps over the lazy dog";
    println!("Pangram {}", pangram);

    println!("Word in revese");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }

    let chars_to_trim: &[char] = &[' ', ','];
    println!("String {}", string);
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used chars {}", trimmed_str);

    let alice = String::from("I like dog");
    let bob: String = alice.replace("dog", "cat");

    println!("Alice {}", alice);
    println!("Bob {}", bob);
}

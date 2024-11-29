fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");

    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the concatenated_string
    println!("{}", concatenated_string);
}


fn concatenate_strings(s1: &str, s2: &str) -> String {
    // Initialize new mutable string
    let mut result = String::new();

    // Concat strings by pushing in result string
    result.push_str(s1);
    result.push_str(s2);

    // Return result
    result
}
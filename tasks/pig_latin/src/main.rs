fn pig_latin(text: &String) -> String {
    const VOWELS: [char; 6+9] = ['e', 'y', 'u', 'i', 'o', 'a', 
    'у', 'е', 'ы', 'а', 'о', 'э', 'я', 'и', 'ю', ];

    let words = text.split_whitespace();
    let mut result = String::new();
    for word in words {
        let chars: Vec<char> = word.chars().collect();
        println!("{chars:?}");  
        let first_char = chars[0];
        println!("{:?}", word.chars());
        if VOWELS.contains(&first_char) {
            result += word;
            result += "-hay";
        } else {
            result += &format!("{}-{}ey", &String::from_iter(&chars[1..]), first_char);
        }
        result += " ";
    }
    result
}

fn main() {
    // let text = String::from("всем привет меня завут даниил я очень люблю есть яблоки");
    let text = String::from("hello world my name is daniel i love eat apples");
    let pig_text = pig_latin(&text);
    println!("{pig_text}");
}

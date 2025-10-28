static MAPPING: [&str; 26] = [
    "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India", "Juliett",
    "Kilo", "Lima", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo", "Sierra", "Tango",
    "Uniform", "Victor", "Whiskey", "Xray", "Yankee", "Zulu",
];

fn map_to_nato(input: &str) -> String {
    let input = input.to_lowercase();

    input
        .bytes()
        .filter_map(|b| MAPPING.get((b - b'a') as usize).copied())
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    std::env::args().skip(1).for_each(|input| {
        let words = map_to_nato(&input);
        println!("{words}");
    });
}

#[cfg(test)]
mod tests {
    use crate::map_to_nato;

    #[test]
    fn it_does_it() {
        assert_eq!(
            map_to_nato("hello"),
            "Hotel Echo Lima Lima Oscar".to_string()
        );
    }
}

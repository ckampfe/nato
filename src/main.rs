static MAPPING: [(u8, &str); 26] = [
    (b'a', "Alfa"),
    (b'b', "Bravo"),
    (b'c', "Charlie"),
    (b'd', "Delta"),
    (b'e', "Echo"),
    (b'f', "Foxtrot"),
    (b'g', "Golf"),
    (b'h', "Hotel"),
    (b'i', "India"),
    (b'j', "Juliett"),
    (b'k', "Kilo"),
    (b'l', "Lima"),
    (b'm', "Mike"),
    (b'n', "November"),
    (b'o', "Oscar"),
    (b'p', "Papa"),
    (b'q', "Quebec"),
    (b'r', "Romeo"),
    (b's', "Sierra"),
    (b't', "Tango"),
    (b'u', "Uniform"),
    (b'v', "Victor"),
    (b'w', "Whiskey"),
    (b'x', "Xray"),
    (b'y', "Yankee"),
    (b'z', "Zulu"),
];

fn main() {
    std::env::args().skip(1).for_each(|input| {
        let input = input.to_lowercase();

        let words = input
            .bytes()
            .map(|b| {
                MAPPING
                    .binary_search_by_key(&b, |(t, _s)| *t)
                    .map(|i| MAPPING[i].1)
                    .expect("inputs must be [A-Za-z]")
            })
            .collect::<Vec<&str>>()
            .join(" ");

        println!("{words}");
    });
}

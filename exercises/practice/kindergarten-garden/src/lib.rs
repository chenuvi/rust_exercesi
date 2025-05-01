const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let plant_from_char = |c: char| -> &'static str {
        match c {
            'R' => "radishes",
            'C' => "clover",
            'G' => "grass",
            'V' => "violets",
            _ => "",
        }
    };

    let cup_index = STUDENTS.iter().position(|&s| s== student).unwrap() * 2;
    let lines = diagram.lines();
    println!("lines:, {:?}", lines);

    lines.flat_map(|line|  {
        line[cup_index..=cup_index+ 1].chars().map(plant_from_char)
    }).collect()
}

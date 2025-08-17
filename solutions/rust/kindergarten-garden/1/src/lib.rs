pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut types_plant = vec![];

    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let student_index = students.iter().position(|&s| s == student).unwrap();

    let rows: Vec<&str> = diagram.split('\n').collect();

    let start_index = student_index * 2;

    for row in rows {
        let chars = row.chars().skip(start_index).take(2).collect::<Vec<char>>();
        for c in chars {
            match c {
                'G' => types_plant.push("grass"),
                'C' => types_plant.push("clover"),
                'R' => types_plant.push("radishes"),
                'V' => types_plant.push("violets"),
                _ => return vec![],
            }
        }
    }
    types_plant
}

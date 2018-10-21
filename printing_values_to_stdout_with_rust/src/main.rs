#[derive(Debug)]
struct Soldier {
    name: String,
    rank: String,
    age: i32,
}

#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
    level: String,
}

fn main() {
    let john_rambo = Soldier{
        name: "John Rambo".to_string(),
        rank: "SSG".to_string(),
        age: 31,
    };

    println!("{:?}", john_rambo);

    let new_student = Student {
        name: "Billy Madison".to_string(),
        age: 27,
        level: "Sophomore".to_string(),
    };

    println!("{student:?}", student=new_student);

    println!("{:#?}", john_rambo);
}

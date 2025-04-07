// Define the Student struct
struct Student {
    major: String,
}

// First-order function: assigns a major to a single student
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

// Higher-order function: updates all students using a behavior function
fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String)) -> Vec<Student> {
    for student in &mut collection {
        behavior(student, "Electrical Engineer".to_string());
    }
    collection
}

fn main() {
    // Create a vector of students with empty majors
    let students = vec![
        Student { major: "".to_string() },
        Student { major: "".to_string() },
        Student { major: "".to_string() },
    ];

    // Update all students using the higher-order function
    let updated_students = update_majors(students, assign_major);

    // Print the updated majors
    for (i, student) in updated_students.iter().enumerate() {
        println!("Student {}: Major = {}", i + 1, student.major);
    }
}

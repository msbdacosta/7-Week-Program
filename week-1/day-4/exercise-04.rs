// Exercise 4: Real-Wold Function -- Split Bill

fn split_bill(total: f64, people: u32) -> f64 {
    total / people as f64
}

fn main() {
    let total_amount = 120.0;
    let num_people = 4;

    let amount_per_person = split_bill(total_amount, num_people);
    println!("Each person should pay: ${:.2}", amount_per_person);
}
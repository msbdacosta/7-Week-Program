// Exercise 3: Loop through list

let fruits = ["Blueberry", "Orange", "Banana", "Apple", "Grape"];

for fruit in fruits {
    if fruit.len() > 6 {
        println!("{}", fruit)
    }
}
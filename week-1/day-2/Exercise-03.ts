// Exercise 3: Looping Over a List

const animals: string[] = [
  "Giraffe",
  "Dog",
  "Elephant",
  "Eagle",
  "Zebra",
  "Cat",
  "Triceraptor",
];
for (const animal of animals) {
  if (animal.length > 6) {
    console.log(animal);
  }
}

// Exercise 5: Challenge — Mini Quiz Game

const question = "What is the capital of France";
const correctAnswer = "Paris";

const userAnswer = prompt(question);

if (userAnswer?.trim().toLowerCase() === correctAnswer.toLowerCase()) {
  alert("Correct");
} else {
  alert("Incorrect. The correct answer is Paris.");
}

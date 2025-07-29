# Exercise 5: Challenge — Number Guessing
import random

print("Welcome to the Guessing Game!")

secret_num = random.randint(1, 10)
guess = int(input("Guess the random number: "))

while guess != secret_num:

    if guess < secret_num:
        print("Too low!")
    elif guess > secret_num:
        print("Too high!")

    guess = int(input("Try again: "))

print(f"You got it! The secret number it's {secret_num}!")

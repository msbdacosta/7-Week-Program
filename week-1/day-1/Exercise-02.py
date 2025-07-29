# Exercise 2: Conditional Greeting
name2 = input("What's your name? ")
age2 = int(input("How old are you? "))

if age2 >= 20:
    print(f"{name2} is an Adult")
elif age2 >= 13:
    print(f"{name2} is a Teenager")
else:
    print(f"{name2} is a Child")

# Exercise 4: Real-world Example — Bill Splitter
def split_bill(total, people):
    splitted = total / people 
    return f"The price each person have to pay is {splitted}"

print(split_bill(100, 4))

iterations = int(input("Please enter a number: "))
a = 7.3
print(f"Number {a} is equal to 1")

for i in range(iterations - 1):
    i += 1
    result = (a - a % 1.0) / 10.0 + (a % 1.0) * 10.0

    if result > a:
        number = result - a
    else:
        number = a - result

    number = round(number * 10.0) / 10.0
    a = number
    print(f"Number {i + 1} is equal to {number}")


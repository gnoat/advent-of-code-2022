from os import system, name
from time import sleep


def clear():
    if "nt" in name:
        system("cls")
    else:
        system("clear")

# Run the Rust solution and capture the stdout
system("cargo run > motion.txt")

# Read output from Rust solution
with open("motion.txt", "r") as f:
    l = f.read()

# Iterate over output from Rust solution and visualize
for img in l.split("\n\n"):
    if "It takes" not in img:
        clear()
        print(img)
        sleep(.02)
    else:
        print("\n[info]", img.strip())
        print(" - Press enter to proceed - ")
        input()
        clear()
system("rm motion.txt")

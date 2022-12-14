from os import system
from time import sleep

system("cargo run > motion.txt")
with open("motion.txt", "r") as f:
    l = f.read()
for img in l.split("\n\n"):
    if "It takes" not in img:
        system("clear")
        print(img)
        sleep(.025)
    else:
        print("\n[info]", img.strip())
        print(" - Press enter to proceed - ")
        input()
        system("clear")
system("rm motion.txt")

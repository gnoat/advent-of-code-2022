from typing import Optional


"""Solved the problem twice below.  Once in a Pythonic way (which I don't like as much)
and once as a one-liner."""

# Pythonic way
def main(s: str, window_size: int = 4) -> Optional[int]:
    for idx in range(window_size, len(s)):
        window = s[idx - window_size: idx]
        if len(window) == len(set(window)):
            return idx


# One liner
main_one_line = lambda s, n: next(idx for idx in range(n, len(s)) if len(s[idx - n: idx]) == len(set(s[idx - n: idx])))


if __name__ == "__main__":
    with open("data.txt", "r") as f:
        data = f.read()
    print(f"The signal stops on index {main(data, 4)} for part one.")
    print(f"[One Liner] The signal stops on index {main_one_line(data, 4)} for part one.")
    print(f"The signal stops on index {main(data, 14)} for part two.")
    print(f"[One Liner] The signal stops on index {main_one_line(data, 14)} for part two.")

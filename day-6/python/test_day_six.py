import pytest
from main import main, main_one_line


def test_part_one():
    test_map = {
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb": 7,
            "bvwbjplbgvbhsrlpgdmjqwftvncz": 5,
            "nppdvjthqldpwncqszvftbrmjlhg": 6,
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg": 10,
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw": 11
            }

    for test_string, expected in test_map.items():
        assert main(test_string) == expected
        assert main_one_line(test_string, 4) == expected


def test_part_two():
    test_map = {
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb": 19,
            "bvwbjplbgvbhsrlpgdmjqwftvncz": 23,
            "nppdvjthqldpwncqszvftbrmjlhg": 23,
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg": 29,
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw": 26
            }

    for test_string, expected in test_map.items():
        assert main(test_string, 14) == expected
        assert main_one_line(test_string, 14) == expected

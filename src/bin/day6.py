from collections import deque
from pathlib import Path


def part_1(path: Path) -> None:
    input_text = path.read_text()
    window_size = 4
    i = window_size
    while i < len(input_text):
        window = input_text[i - 4 : i]
        if len(set(window)) == 4:
            print(i)
            break
        i += 1


def part_2(path: Path) -> None:
    ...


def main():
    part_1(Path('src/input/day6_1'))
    # part_2(Path('src/input/day6_2'))


if __name__ == '__main__':
    main()

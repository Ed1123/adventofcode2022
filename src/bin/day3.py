from pathlib import Path


def decode_item(item: str) -> int:
    n = ord(item)
    if item.isupper():
        return n - 38
    else:
        return n - 96


def part_1(path: Path) -> None:
    input_text = path.read_text()
    common_items = []
    for line in input_text.splitlines():
        half = len(line) // 2
        first_half, second_half = line[:half], line[half:]
        common_items.append(set(first_half).intersection(set(second_half)).pop())

    print(sum(decode_item(item) for item in common_items))


def part_2(path: Path) -> None:
    lines = path.read_text().splitlines()
    badges = []
    for i in range(len(lines) // 3):
        group = lines[i * 3 : (i + 1) * 3]
        # print(group)
        items = [set(line) for line in group]
        badges.append(items[0].intersection(items[1]).intersection(items[2]).pop())

    print(sum(decode_item(badge) for badge in badges))


def main():
    part_1(Path('src/input/day3_1'))
    part_2(Path('src/input/day3_2'))


if __name__ == '__main__':
    main()

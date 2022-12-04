from pathlib import Path


def decode_item(item: str) -> int:
    n = ord(item)
    if item.isupper():
        return n - 38
    else:
        return n - 96


def main():
    path = Path('src/input/day3_1')
    input_text = path.read_text()
    common_items = []
    for line in input_text.splitlines():
        half = len(line) // 2
        first_half, second_half = line[:half], line[half:]
        common_items.append(set(first_half).intersection(set(second_half)).pop())

    print(sum(decode_item(item) for item in common_items))


if __name__ == '__main__':
    main()

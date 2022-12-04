from pathlib import Path


def part_1(path: Path) -> None:
    lines = path.read_text().splitlines()
    count = 0
    for line in lines:
        ((i1, j1), (i2, j2)) = [
            map(int, _range.split('-')) for _range in line.split(',')
        ]
        # print(i1, j1, i2, j2)
        range_1 = set(range(i1, j1 + 1))
        range_2 = set(range(i2, j2 + 1))
        if range_1.issuperset(range_2) or range_1.issubset(range_2):
            count += 1
    print(count)


def part_2(path: Path) -> None:
    lines = path.read_text().splitlines()
    count = 0
    for line in lines:
        ((i1, j1), (i2, j2)) = [
            map(int, _range.split('-')) for _range in line.split(',')
        ]
        # print(i1, j1, i2, j2)
        range_1 = set(range(i1, j1 + 1))
        range_2 = set(range(i2, j2 + 1))
        # print(range_1.intersection(range_2))
        if len(range_1.intersection(range_2)) > 0:
            count += 1
    print(count)


def main():
    part_1(Path('src/input/day4_1'))
    part_2(Path('src/input/day4_1'))


if __name__ == '__main__':
    main()

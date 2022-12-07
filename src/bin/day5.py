from collections import deque
from pathlib import Path


def part_1(path: Path) -> None:
    crates_stacks, moves = path.read_text().split('\n\n')

    # Crates in deques (double-ended queue)
    # To populate append to the right
    # To move pop from the left and insert to the right
    crates_rows = crates_stacks.splitlines(keepends=True)
    piles = [deque() for _ in range(len(crates_rows[0]) // 4)]
    for crate_row in crates_rows[:-1]:
        for i, pile in enumerate(piles):
            crate = crate_row[i * 4 : (i + 1) * 4][1:2]
            if crate != ' ':
                pile.append(crate)

    # Making the moves
    for moves_row in moves.splitlines():
        _, crates_count, _, from_pile, _, to_pile = moves_row.split(' ')
        from_pile = int(from_pile) - 1
        to_pile = int(to_pile) - 1
        for _ in range(int(crates_count)):
            crate = piles[from_pile].popleft()
            piles[to_pile].appendleft(crate)

    for pile in piles:
        print(pile.popleft(), end='')
    print()


def part_2(path: Path) -> None:
    crates_stacks, moves = path.read_text().split('\n\n')

    # Crates in deques (double-ended queue)
    # To populate append to the right
    # To move pop from the left and insert to the right
    crates_rows = crates_stacks.splitlines(keepends=True)
    piles = [deque() for _ in range(len(crates_rows[0]) // 4)]
    for crate_row in crates_rows[:-1]:
        for i, pile in enumerate(piles):
            crate = crate_row[i * 4 : (i + 1) * 4][1:2]
            if crate != ' ':
                pile.append(crate)

    # Making the moves
    for moves_row in moves.splitlines():
        _, crates_count, _, from_pile, _, to_pile = moves_row.split(' ')
        from_pile = int(from_pile) - 1
        to_pile = int(to_pile) - 1

        cratemover = []
        for _ in range(int(crates_count)):
            crate = piles[from_pile].popleft()
            cratemover.append(crate)

        while len(cratemover) != 0:
            piles[to_pile].appendleft((cratemover.pop()))

    for pile in piles:
        print(pile.popleft(), end='')
    print()


def main():
    part_1(Path('src/input/day5_1'))
    part_2(Path('src/input/day5_1'))


if __name__ == '__main__':
    main()

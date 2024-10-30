def is_numeric(c: str) -> bool:
    return c in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]


def calculate_day_a_line(line: str) -> int:
    start: int | None = None
    end: int | None = None
    for c in line:
        if is_numeric(c):
            if start is None:
                start = int(c)
            end = int(c)
    assert start is not None
    assert end is not None
    return start * 10 + end


def day_a(filepath: str) -> int:
    with open(filepath, "r") as f:
        lines: list[str] = f.readlines()
    return sum([calculate_day_a_line(line) for line in lines])


MAPPING: dict[str, int] = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}


def get_val_day_b(line: str) -> int | None:
    if is_numeric(line[0]):
        return int(line[0])
    for key in MAPPING:
        if line.startswith(key):
            return MAPPING[key]
    return None


def calculate_day_b_line(line: str) -> int:
    start: int | None = None
    end: int | None = None
    for i in range(len(line)):
        val = get_val_day_b(line[i:])
        if val is not None:
            if start is None:
                start = val
            end = val
    assert start is not None
    assert end is not None
    return start * 10 + end


def day_b(filepath: str) -> int:
    with open(filepath, "r") as f:
        lines: list[str] = f.readlines()
    return sum([calculate_day_b_line(line) for line in lines])

from pathlib import Path
from sys import exit


def concat_versions(versions: list[tuple[str, int]], i: int) -> tuple[str, int]:
    j = i
    version = versions[j]
    str = version[0]

    while j < len(versions) - 1 and version[1] == versions[j + 1][1]:
        j += 1
        version = versions[j]
        str += f"/{version[0]}"

    j += 1

    return (str, j)


def main():
    lines = Path("versions.txt").read_text().splitlines()

    versions: list[tuple[str, int]] = []

    for line in lines:
        split = line.split(" = ")
        name = split[0]
        version = int(split[1])
        versions.append((name, version))

    func = f"""    match version {{
            i32::MIN..=-1 => "Unknown","""

    i = 0

    while i < len(versions):
        if i == 0:
            previous_tuple = ("", 0)
        else:
            previous_tuple = versions[i - 1]
        current = versions[i]

        if i < len(versions) - 1:
            next_tuple = versions[i + 1]
        else:
            next_tuple = versions[-1]

        if previous_tuple[1] > current[1] or current[1] > next_tuple[1]:
            print(f"ERROR: not ordered (line {i+1})")
            exit(1)

        next_version = next_tuple[1]

        name = current[0]
        version = current[1]

        if version == next_version:
            concat, new_i = concat_versions(versions, i)
            func += f"""
            {version} => "{concat}","""
            i = new_i
            continue

        i += 1

        func += f"""
            {version} => "{name}","""

        if version == next_version - 1 or version + 1 == next_version:
            continue

        if version + 1 == next_version - 1:
            func += f"""
            {version+1} => "{name} (or newer)","""
            continue

        func += f"""
            {version+1}..={next_version-1} => "{name} (or newer)","""

    func += f"""
            _ => "{versions[-1][0]} (or newer)"
        }}"""

    print(func)


if __name__ == "__main__":
    main()

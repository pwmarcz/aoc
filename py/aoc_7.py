import sys


def main():
    dirs = {}
    path = []
    for line in sys.stdin:
        line = line.rstrip()
        assert len(line) > 0

        if line == '$ cd /':
            path.clear()
        elif line == '$ cd ..':
            path.pop()
        elif line.startswith('$ cd '):
            dirname = line[len('$ cd '):]
            path.append(dirname)
        elif line == '$ ls':
            pass
        elif line.startswith('dir '):
            pass
        else:
            assert line[0].isdigit(), line
            size, _space, name = line.partition(' ')
            size = int(size)
            for i in range(len(path) + 1):
                p = tuple(path[:i])
                dirs.setdefault(p, 0)
                dirs[p] += size

    total = 0
    for path, size in dirs.items():
        if size <= 100000:
            total += size
    print(total)

    disk_size = 70_000_000
    need_space = 30_000_000
    cur_space = disk_size - dirs[()]
    smallest = min(
        size for size in dirs.values()
        if cur_space + size > need_space
    )
    print(smallest)


if __name__ == '__main__':
    main()

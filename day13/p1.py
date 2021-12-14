def pp(xys):
    w, h = max(x for x, _ in xys), max(y for _, y in xys)
    out = ""
    for y in range(h + 3):
        for x in range(w + 3):
            out += "." if (x, y) not in xys else "◼"
        out += "\n"
    print(out)


if __name__ == '__main__':
    with open("../inputs/day13.txt") as fi:
        points_, folds_ = fi.read().split("\n\n", 1)
    points = [tuple([int(i) for i in line.split(",")]) for line in points_.split("\n")]
    folds = [f.replace("fold along ", "").split("=") for f in folds_.split("\n")]
    folds = [(axis, int(n)) for axis, n in folds]

    # pp(points)
    origami = list(points)
    for axis, n in folds:
        if axis == "x":
            origami = [(2 * n - xx if xx > n else xx, yy) for xx, yy in origami]
        else:
            origami = [(xx, 2 * n - yy if yy > n else yy) for xx, yy in origami]
    pp(origami)

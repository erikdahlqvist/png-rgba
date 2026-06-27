import png

W = 64
H = 64

def gen_rgba(depth):
    rows = []
    max_value = 2**depth - 1

    for y in range(H):
        row = []

        for x in range(W):
            r = (x * max_value) // (W - 1)
            g = (y * max_value) // (H - 1)
            b = max_value // 2
            a = ((x + y) * max_value) // (W + H - 2)
            row.extend([r, g, b, a])

        rows.append(row)

    with open(f"rgba{depth}-{W}x{H}.png", "wb") as f:
        writer = png.Writer(
            width=W,
            height=H,
            alpha=True,
            bitdepth=depth,
            greyscale=False,
        )
        writer.write(f, rows)

def gen_rgb(depth):
    rows = []
    max_value = 2**depth - 1

    for y in range(H):
        row = []

        for x in range(W):
            r = (x * max_value) // (W - 1)
            g = (y * max_value) // (H - 1)
            b = max_value // 2
            row.extend([r, g, b])

        rows.append(row)

    with open(f"rgb{depth}-{W}x{H}.png", "wb") as f:
        writer = png.Writer(
            width=W,
            height=H,
            alpha=False,
            bitdepth=depth,
            greyscale=False,
        )
        writer.write(f, rows)


gen_rgba(8)
gen_rgba(16)
gen_rgb(8)
gen_rgb(16)
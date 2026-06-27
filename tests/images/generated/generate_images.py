import png

W = 64
H = 64

def gen_rgb(depth, alpha):
    rows = []
    max_value = 2**depth - 1

    print_a = "a" if alpha else ""

    for y in range(H):
        row = []

        for x in range(W):
            r = (x * max_value) // (W - 1)
            g = (y * max_value) // (H - 1)
            b = max_value // 2

            if alpha:
                a = ((x + y) * max_value) // (W + H - 2)
                row.extend([r, g, b, a])
            else:
                row.extend([r, g, b])

        rows.append(row)

    with open(f"rgb{print_a}{depth}-{W}x{H}.png", "wb") as f:
        writer = png.Writer(
            width=W,
            height=H,
            alpha=alpha,
            bitdepth=depth,
            greyscale=False,
        )
        writer.write(f, rows)

gen_rgb(8, True)
gen_rgb(16, True)
gen_rgb(8, False)
gen_rgb(16, False)
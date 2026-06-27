import png

W = 64
H = 64

rows = []

for y in range(H):
    row = []

    for x in range(W):
        r = (x * 255) // (W - 1)
        g = (y * 255) // (H - 1)
        b = 127
        a = ((x + y) * 255) // (W + H - 2)

        row.extend([r, g, b, a])

    rows.append(row)

with open("rgba8-64x64.png", "wb") as f:
    writer = png.Writer(
        width=W,
        height=H,
        alpha=True,
        bitdepth=8,
        greyscale=False,
    )
    writer.write(f, rows)

#!/usr/bin/env python3
"""Rebuild the Nivra packaged icons from the brand masters. Development only.

Renders `assets/brand/nivra.svg` into the hicolor theme, the Windows executable icon and
the macOS icon set, and copies the scalable vector sources. Rasterizing needs either the
`resvg` 0.45.1 command line tool on PATH (or `--resvg <path>`) or the `resvg_py` module:

    cargo install resvg --version 0.45.1 --root /tmp/resvg-tool
import re
    python3 tools/generate-brand-icons.py --resvg /tmp/resvg-tool/bin/resvg
"""

import argparse
import re
import shutil
import struct
import subprocess
import sys
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BRAND = ROOT / "assets" / "brand"

HICOLOR_SIZES = (16, 22, 24, 32, 48, 64, 96, 128, 256, 512, 1024)
ICO_SIZES = (16, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 128, 192, 256)
ICNS_ENTRIES = (
    ("icp4", 16), ("ic11", 32), ("icp5", 32), ("ic12", 64), ("ic07", 128),
    ("ic13", 256), ("ic08", 256), ("ic14", 512), ("ic09", 512), ("ic10", 1024),
)


class Rasterizer:
    """Render an SVG to PNG bytes with resvg, by CLI when present, else via resvg_py."""

    def __init__(self, executable):
        self.executable = shutil.which(executable) if executable else None
        self.module = None
        if not self.executable:
            try:
                import resvg_py
            except ImportError:
                resvg_py = None
            if resvg_py is None:
                raise SystemExit(
                    "resvg is required: install the resvg 0.45.1 CLI or the resvg_py module"
                )
            self.module = resvg_py

    def png(self, svg, size):
        if self.executable:
            import tempfile
            with tempfile.TemporaryDirectory(prefix="nivra-icon-") as directory:
                out = Path(directory) / "out.png"
                subprocess.run(
                    [self.executable, "--width", str(size), "--height", str(size),
                     str(svg), str(out)],
                    check=True,
                )
                return out.read_bytes()
        return self.module.svg_to_bytes(svg_path=str(svg), width=size, height=size)


def decode_rgba(data):
    """Decode a non-interlaced 8 bit RGBA PNG into (width, height, rows of bytearray)."""
    if data[:8] != bytes([137, 80, 78, 71, 13, 10, 26, 10]):
        raise ValueError("not a PNG")
    header = None
    idat = bytearray()
    offset = 8
    while offset < len(data):
        length, kind = struct.unpack(">I4s", data[offset:offset + 8])
        body = data[offset + 8:offset + 8 + length]
        if kind == b"IHDR":
            header = struct.unpack(">IIBBBBB", body)
        elif kind == b"IDAT":
            idat += body
        elif kind == b"IEND":
            break
        offset += 12 + length
    width, height, depth, colour, comp, filt, interlace = header
    if (depth, colour, comp, filt, interlace) != (8, 6, 0, 0, 0):
        raise ValueError("expected a non-interlaced 8 bit RGBA PNG")
    raw = zlib.decompress(bytes(idat))
    stride = width * 4
    rows = []
    previous = bytearray(stride)
    pos = 0
    for _ in range(height):
        method = raw[pos]
        pos += 1
        line = bytearray(raw[pos:pos + stride])
        pos += stride
        if method == 1:
            for i in range(4, stride):
                line[i] = (line[i] + line[i - 4]) & 0xFF
        elif method == 2:
            for i in range(stride):
                line[i] = (line[i] + previous[i]) & 0xFF
        elif method == 3:
            for i in range(stride):
                left = line[i - 4] if i >= 4 else 0
                line[i] = (line[i] + ((left + previous[i]) >> 1)) & 0xFF
        elif method == 4:
            for i in range(stride):
                a = line[i - 4] if i >= 4 else 0
                b = previous[i]
                c = previous[i - 4] if i >= 4 else 0
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                pred = a if (pa <= pb and pa <= pc) else (b if pb <= pc else c)
                line[i] = (line[i] + pred) & 0xFF
        elif method != 0:
            raise ValueError(f"unknown PNG filter {method}")
        rows.append(line)
        previous = line
    return width, height, rows


def dib(size, rows):
    """Encode an RGBA image as the BMP DIB an .ico entry needs (BGRA, bottom-up)."""
    height = len(rows)
    width = len(rows[0]) // 4
    if (width, height) != (size, size):
        raise ValueError(f"expected {size}x{size}, rendered {width}x{height}")
    xor = bytearray()
    for line in reversed(rows):
        for x in range(width):
            i = x * 4
            r, g, b, a = line[i], line[i + 1], line[i + 2], line[i + 3]
            xor += bytes((b, g, r, a))
    mask_stride = ((width + 31) // 32) * 4
    mask = bytes(mask_stride * height)
    pixels = bytes(xor) + mask
    header = struct.pack(
        "<IiiHHIIiiII", 40, width, height * 2, 1, 32, 0, len(pixels), 0, 0, 0, 0
    )
    return header + pixels


def build_ico(frames):
    """frames is a list of (size, payload) with DIB payloads below 256 and PNG at 256."""
    header = struct.pack("<HHH", 0, 1, len(frames))
    offset = 6 + 16 * len(frames)
    entries = []
    for size, payload in frames:
        side = 0 if size >= 256 else size
        entries.append(struct.pack(
            "<BBBBHHII", side, side, 0, 0, 1, 32, len(payload), offset
        ))
        offset += len(payload)
    return header + b"".join(entries) + b"".join(payload for _, payload in frames)


def build_icns(entries):
    """entries is a list of (four character type, png bytes)."""
    body = b"icns" + b""
    body += struct.pack(">I", 8 + 8 + len(entries) * 8 + sum(8 + len(d) for _, d in entries))
    body += b"TOC " + struct.pack(">I", 8 + len(entries) * 8)
    for kind, payload in entries:
        body += kind + struct.pack(">I", 8 + len(payload))
    for kind, payload in entries:
        body += kind + struct.pack(">I", 8 + len(payload)) + payload
    return body


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--resvg", default="resvg", help="path to the resvg executable")
    args = parser.parse_args()
    raster = Rasterizer(args.resvg)
    master = BRAND / "nivra.svg"
    if not master.exists():
        raise SystemExit(f"missing {master}")

    hicolor = ROOT / "packaging" / "linux" / "hicolor"
    for size in HICOLOR_SIZES:
        target = hicolor / f"{size}x{size}" / "apps" / "nivra.png"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(raster.png(master, size))
        print(f"{target.relative_to(ROOT)} {target.stat().st_size} bytes")

    scalable = hicolor / "scalable" / "apps"
    scalable.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(master, scalable / "nivra.svg")
    # The symbolic icon is the mark centred on a 1024 canvas. The mark is authored in
    # its own square box, so scaling it about the origin centres it; the transform is
    # baked into the path so the file stays plain geometry with no wrapper group.
    mark = (BRAND / "nivra-mark.svg").read_text(encoding="utf-8")
    box = [float(v) for v in mark.split("viewBox=" + chr(34), 1)[1].split(chr(34), 1)[0].split()]
    scale = 1024.0 / box[2]
    body = mark[mark.index("<path"):mark.rindex("</svg>")]
    body = body.replace("fill=" + chr(34) + "#fff" + chr(34), "fill=" + chr(34) + "#000000" + chr(34))
    quoted = "d=" + chr(34)
    start = body.index(quoted) + len(quoted)
    end = body.index(chr(34), start)
    scaled = re.sub(
        r"-?[0-9]+(?:\.[0-9]+)?",
        lambda match: "%.2f" % (float(match.group(0)) * scale),
        body[start:end],
    )
    body = body[:start] + scaled + body[end:]
    head = "".join([
        "<?xml version=" + chr(39) + "1.0" + chr(39) + " encoding=" + chr(39) + "UTF-8" + chr(39) + "?>" + chr(10),
        "<svg xmlns=" + chr(39) + "http://www.w3.org/2000/svg" + chr(39) + " width=" + chr(39) + "1024" + chr(39) + " height=" + chr(39) + "1024" + chr(39) + " ",
        "viewBox=" + chr(39) + "0 0 1024 1024" + chr(39) + " fill=" + chr(39) + "none" + chr(39) + ">" + chr(10),
    ])
    symbolic = head + body + "</svg>" + chr(10)
    (scalable / "nivra-symbolic.svg").write_text(symbolic, encoding="utf-8", newline=chr(10))

    windows = ROOT / "packaging" / "windows"
    frames = []
    for size in ICO_SIZES:
        png = raster.png(master, size)
        frames.append((size, png if size >= 256 else dib(size, decode_rgba(png)[2])))
    (windows / "Nivra.ico").write_bytes(build_ico(frames))
    (windows / "nivra.png").write_bytes(raster.png(master, 256))

    macos = ROOT / "packaging" / "macos"
    (macos / "Nivra.icns").write_bytes(
        build_icns([(kind.encode("ascii"), raster.png(master, size)) for kind, size in ICNS_ENTRIES])
    )
    for name in ["Nivra.ico", "nivra.png", "../macos/Nivra.icns"]:
        print(name, (ROOT / "packaging" / "windows" / name).stat().st_size, "bytes")


if __name__ == "__main__":
    main()

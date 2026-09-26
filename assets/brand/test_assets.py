#!/usr/bin/env python3
"""Brand asset regression checks for the Nivra marks; never launches the app.

The packaged artwork has to survive being composited onto light and dark surfaces, so the
PNGs are measured byte by byte: four channels, transparent corners, an opaque centre and no
dark fringe left behind by the black matte the master artwork was drawn on. The SVGs are
parsed, and the mark view box is checked for squareness because tools/generate-icons.py
scales a repository mark by its view box width into a fixed 64 pixel atlas cell.
"""

from pathlib import Path
import struct
import unittest
import xml.etree.ElementTree as ET
import zlib

BRAND = Path(__file__).resolve().parent
SVG_NS = '{http://www.w3.org/2000/svg}'

CANVAS = 1024
TRAY = 72

# A fringe pixel may not fall below this share of the plate median luminance,
# and at most this share of the fringe is allowed to do so.
BLACK_MATTE_FRACTION = 0.25
BLACK_MATTE_BUDGET = 0.02


def read_png(path):
    """Decode a non-interlaced 8 bit PNG into width, height, channels and rows."""
    data = path.read_bytes()
    if data[:8] != bytes([137, 80, 78, 71, 13, 10, 26, 10]):
        raise ValueError(f'{path} is not a PNG')
    header = None
    idat = bytearray()
    offset = 8
    while offset < len(data):
        length, kind = struct.unpack('>I4s', data[offset:offset + 8])
        body = data[offset + 8:offset + 8 + length]
        if kind == b'IHDR':
            header = struct.unpack('>IIBBBBB', body)
        elif kind == b'IDAT':
            idat += body
        elif kind == b'IEND':
            break
        offset += 12 + length
    if header is None:
        raise ValueError(f'{path} has no IHDR')
    width, height, depth, colour, comp, filt, interlace = header
    if depth != 8 or comp != 0 or filt != 0 or interlace != 0:
        raise ValueError(f'{path} uses an unsupported PNG configuration')
    channels = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}.get(colour)
    if channels is None:
        raise ValueError(f'{path} has an unknown colour type {colour}')
    raw = zlib.decompress(bytes(idat))
    stride = width * channels
    rows = []
    previous = bytearray(stride)
    pos = 0
    for _ in range(height):
        method = raw[pos]
        pos += 1
        line = bytearray(raw[pos:pos + stride])
        pos += stride
        if method == 1:
            for i in range(channels, stride):
                line[i] = (line[i] + line[i - channels]) & 0xFF
        elif method == 2:
            for i in range(stride):
                line[i] = (line[i] + previous[i]) & 0xFF
        elif method == 3:
            for i in range(stride):
                left = line[i - channels] if i >= channels else 0
                line[i] = (line[i] + ((left + previous[i]) >> 1)) & 0xFF
        elif method == 4:
            for i in range(stride):
                a = line[i - channels] if i >= channels else 0
                b = previous[i]
                c = previous[i - channels] if i >= channels else 0
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                pred = a if (pa <= pb and pa <= pc) else (b if pb <= pc else c)
                line[i] = (line[i] + pred) & 0xFF
        elif method != 0:
            raise ValueError(f'{path} uses unknown filter {method}')
        rows.append(line)
        previous = line
    return width, height, channels, rows


def rgba_rows(path):
    """Decode an 8 bit RGBA PNG and yield red, green, blue and alpha tuples."""
    width, _, channels, rows = read_png(path)
    if channels != 4:
        raise AssertionError(f'{path.name} has {channels} channels, expected 4 (RGBA)')
    for line in rows:
        for x in range(width):
            i = x * 4
            yield line[i], line[i + 1], line[i + 2], line[i + 3]


def luminance(red, green, blue):
    return 0.2126 * red + 0.7152 * green + 0.0722 * blue


class BrandAssetTest(unittest.TestCase):
    def test_master_png_is_rgba_sized_for_the_canvas(self):
        width, height, channels, _ = read_png(BRAND / 'nivra-1024.png')
        self.assertEqual((width, height), (CANVAS, CANVAS))
        self.assertEqual(channels, 4, 'nivra-1024.png must carry an alpha channel')

    def test_master_png_corners_are_clear_and_centre_is_opaque(self):
        pixels = list(rgba_rows(BRAND / 'nivra-1024.png'))
        side = CANVAS - 1
        for x, y in [(0, 0), (side, 0), (0, side), (side, side)]:
            self.assertEqual(pixels[y * CANVAS + x][3], 0, f'corner {x},{y} must be transparent')
        centre = CANVAS // 2
        self.assertEqual(pixels[centre * CANVAS + centre][3], 255, 'the centre must be opaque')

    def test_master_png_has_no_dark_matte_fringe(self):
        """A naive cut of the black backed master darkens the rim; de-matte must not."""
        opaque = []
        fringe = []
        for red, green, blue, alpha in rgba_rows(BRAND / 'nivra-1024.png'):
            if alpha == 255:
                opaque.append(luminance(red, green, blue))
            elif alpha > 0:
                fringe.append(luminance(red, green, blue))
        self.assertTrue(opaque, 'the plate has no fully opaque pixel')
        self.assertTrue(fringe, 'the plate edge carries no antialiasing at all')
        opaque.sort()
        median = opaque[len(opaque) // 2]
        threshold = median * BLACK_MATTE_FRACTION
        dark = sum(1 for value in fringe if value < threshold)
        share = dark / len(fringe)
        detail = f'{dark}/{len(fringe)} dark fringe pixels, plate median {median:.1f}'
        self.assertLess(share, BLACK_MATTE_BUDGET, detail)

    def test_tray_png_is_black_on_transparent(self):
        width, height, channels, _ = read_png(BRAND / 'nivra-tray.png')
        self.assertEqual((width, height), (TRAY, TRAY))
        self.assertEqual(channels, 4, 'nivra-tray.png must carry an alpha channel')
        levels = set()
        covered = 0
        for red, green, blue, alpha in rgba_rows(BRAND / 'nivra-tray.png'):
            self.assertEqual((red, green, blue), (0, 0, 0), 'the tray template is tinted by the OS')
            levels.add(alpha)
            covered += 1 if alpha else 0
        self.assertGreater(len(levels), 1, 'the tray image carries no antialiasing')
        self.assertGreater(covered, 0, 'the tray image is empty')

    def test_svgs_parse_and_carry_artwork(self):
        for name in ['nivra.svg', 'nivra-flat.svg', 'nivra-mark.svg']:
            with self.subTest(name=name):
                root = ET.parse(BRAND / name).getroot()
                self.assertEqual(root.tag, SVG_NS + 'svg')
                self.assertIn('viewBox', root.attrib)
                self.assertTrue(root.findall('.//' + SVG_NS + 'path'))

    def test_mark_view_box_is_square(self):
        """generate-icons.py scales a repository mark into a fixed 64 pixel atlas cell."""
        root = ET.parse(BRAND / 'nivra-mark.svg').getroot()
        box = [float(value) for value in root.attrib['viewBox'].split()]
        self.assertEqual(len(box), 4)
        self.assertAlmostEqual(box[2], box[3], places=2, msg='the mark view box must be square')
        self.assertGreater(box[2], 0)

    def test_mark_is_a_solid_white_silhouette(self):
        root = ET.parse(BRAND / 'nivra-mark.svg').getroot()
        paths = root.findall('.//' + SVG_NS + 'path')
        self.assertTrue(paths)
        for path in paths:
            fill = path.attrib.get('fill', root.attrib.get('fill', ''))
            self.assertIn(fill.lower(), ('#fff', '#ffffff', 'white'))


if __name__ == '__main__':
    unittest.main()
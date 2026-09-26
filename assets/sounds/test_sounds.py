#!/usr/bin/env python3
"""Notification sound regression checks; never plays audio or launches the app.

The pack is Nivra's own work, so these tests police the two things that could quietly
reintroduce a borrowed asset: a cue that the real decoder would reject, and any surviving
reference to the retired Discord pack. The decode itself is covered by the Rust test in
`apps/desktop/src/notification_sounds.rs`, which runs the production decoder.
"""

from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[2]
SOUNDS = ROOT / "assets" / "sounds"
PACK = SOUNDS / "nivra"

CUES = (
    "camera-on",
    "current-channel",
    "deafen",
    "incoming-ring",
    "message",
    "mute",
    "outgoing-ring",
    "screen-share-on",
    "undeafen",
    "unmute",
    "user-join",
    "user-leave",
)

MAX_BYTES = 128 * 1024

SCANNED = ("apps", "crates", "packaging", "docs", "assets", "tools", "tests")
TEXT_SUFFIXES = (".rs", ".py", ".cjs", ".md", ".sh", ".json", ".toml", ".yml", ".txt")
RETIRED = re.compile(r"sounds/discord|discord/[a-z-]+\.mp3", re.IGNORECASE)


class NotificationSoundTest(unittest.TestCase):
    def test_every_cue_is_present_exactly_once(self):
        found = sorted(path.stem for path in PACK.glob("*.ogg"))
        self.assertEqual(found, sorted(CUES))

    def test_cues_are_ogg_within_the_encoded_budget(self):
        for cue in CUES:
            with self.subTest(cue=cue):
                path = PACK / f"{cue}.ogg"
                data = path.read_bytes()
                self.assertEqual(data[:4], b"OggS", f"{cue} is not an Ogg stream")
                self.assertLessEqual(len(data), MAX_BYTES, f"{cue} is over 128 KiB")
                self.assertGreater(len(data), 0)

    def test_the_retired_discord_pack_is_gone(self):
        self.assertFalse(
            (SOUNDS / "discord").exists(),
            "the retired Discord sound directory is still present",
        )

    def test_nothing_references_the_retired_discord_pack(self):
        found = []
        for folder in SCANNED:
            for path in sorted((ROOT / folder).rglob("*")):
                if path.suffix not in TEXT_SUFFIXES or not path.is_file():
                    continue
                if path.resolve() == Path(__file__).resolve():
                    continue  # this file names the retired path on purpose
                try:
                    text = path.read_text(encoding="utf-8")
                except (UnicodeDecodeError, OSError):
                    continue
                for number, line in enumerate(text.splitlines(), start=1):
                    if RETIRED.search(line):
                        found.append(f"{path.relative_to(ROOT).as_posix()}:{number}")
        self.assertEqual(found, [], f"stale Discord sound references: {found}")

    def test_third_party_notices_no_longer_claims_discord_sounds(self):
        text = (ROOT / "THIRD_PARTY_NOTICES.md").read_text(encoding="utf-8")
        self.assertNotIn("notification sound pack embeds", text)
        self.assertNotIn("redistribution-permission review", text)


if __name__ == "__main__":
    unittest.main()
# Notification sounds

The `nivra/` files are Nivra's own notification pack. Every cue was commissioned and
produced for this project, so the whole set is covered by this repository's MIT and
Apache-2.0 licences like the rest of the code, and nothing here needs a separate
redistribution grant.

They are Ogg Vorbis, stereo, 48 kHz, each below 128 KiB encoded and six seconds decoded.
Playback converts from the source sample rate; no re-encoding, metadata stripping or
trimming is applied. The longer cues are the two rings, which repeat so a call is still
audible after the clip ends.

| Bundled file | Bytes | SHA-256 |
| --- | --- | --- |
| `camera-on.ogg` | 6153 | `4f736c9f3bfb6989cf9db4bca22a174f6a78ef8e43b58b89723259c427d2c8fa` |
| `current-channel.ogg` | 6428 | `c77b5b4505a6ea1d71b7a89a70b1023c3ea309af08857d967c8ce553e99a9dec` |
| `deafen.ogg` | 6228 | `606ac1cbd387d774a0bfa382819ae67f00ff6ec809f60a6cebf6d06383f6eb29` |
| `incoming-ring.ogg` | 16664 | `f793e925b8ef1d3e078dfce7bcf48be0e01a9a98508e60d11a5f50462a7c7908` |
| `message.ogg` | 6580 | `9d837adc0f1f24fa717bf819b729179fbc73a6a1d36113bb2688c4bb6708a190` |
| `mute.ogg` | 6291 | `385d725527c981a400b2c0f3da622ec38c35b918f329f191175e3b93dba2c584` |
| `outgoing-ring.ogg` | 10533 | `a1152abff6ed0a51934d93d8a4177a9903401785038cbc29c29be1441b9c6e96` |
| `screen-share-on.ogg` | 6979 | `1638547b939a000f9d85c1e85191c47b0a68d7a1f2f480750775ce26cea29620` |
| `undeafen.ogg` | 5865 | `f86225fe3a775e80ab101d1dd4f4b0e5e16be11b3b027e026e8650ce0c87d792` |
| `unmute.ogg` | 6266 | `b68315dad770179ac324e8ae9abf08a4c4385869e812d6ecaad43d3e2f3ae06c` |
| `user-join.ogg` | 6400 | `86765d74a613a5567b0b22b6dea5b5c51b9ab2037ed32c9331aab5868a369277` |
| `user-leave.ogg` | 6415 | `8215d96a16170d4f00ad9cf2361101da151f133f4a7c13d7b22773f96d7f087c` |

The twelve files total 90,802 bytes; each distinct cue is bundled independently. No
network fetch or user-file access occurs during playback.

## Attribution

These sounds are Nivra's own work and carry no third-party attribution requirement.
Nivra is an unofficial Discord client and is not affiliated with or endorsed by
Discord; the cue names describe interface events, not Discord assets.

`apps/desktop/src/notification_sounds.rs` maps each `Sound` variant to one file, and a
test decodes all twelve through the real playback decoder so a bad file cannot ship.
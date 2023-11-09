# Changelog

---
## Upcoming

### Added
- Allowed macron to be read as a long vowel marker.
### Changed
- Changed mapping of comma from single dot to vertical underline / *Thinnas* diacritic.
### Fixed
- Fixed a bug where separate long vowel style would override the Nuquerna setting, even for short vowels.

---
## 1.1.0: 2023-10-03

### Added
- Implemented `--dot-plain` and `--elide-a` CLI options to enable variant behaviors used for Quenya.
- Implemented `Policy` trait to adjust `Glyph` behavior.
- Implemented "silent split" escape sequence. Now, ASCII input string `"etya\ ngoldorin"` will be presented unspaced, like `"etyangoldorin"`, but the `NG` will correctly use initial *Ñoldo* instead of medial *Anga*.
- Added `Æ` and `Œ` as valid Sindarin diphthongs.
### Changed
- Boolean `Glyph::with_*` methods now take inputs.
### Fixed
- Fixed mistake where initial `NW` used *Númen*+*Wilya* instead of *Ñwalmë*.

---
## 1.0.0: 2022-11-19
- Released.

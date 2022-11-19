# TODO

---
## 1.0.0

### CLI
### Docs
### Input
- [x] Normalize diacritics in input
- [x] ~~Support (duo)decimal points OR~~ drop generic in `Numeral`
### Library
- [x] Implement "sequence" numerals
- [x] Redo ZWJ ligature cases
### Output
- [x] Write nonfinal rincë **before** tehtar
### Tests
- [x] Write iterator-focused test suite in `tests/`
- [x] Write numeral-focused tests

---
## 1.1.0

### CLI
### Docs
### Input
### Library
- [ ] Gondor "Full" Mode
- [ ] Flesh out `Policy` to make ligature/rincë rules extensible
### Output
### Tests

---
## Whenever

### CLI
### Docs
### Input
- [ ] Definitively decide what to do wrt "language" vs "mode":
  - Would be very nice to have a dedicated `--language`/`-L` option for frontend clarity. How would this interact with having multiple Sindarin modes?
  - Possible division:
    - Explicit "mode" options (`--quenya`, `--gondor`) become "language" options (`--quenya`, `--sindarin`), each mapping to a "primary mode" for the language
    - Values of `--mode` option (`--mode=quenya`, `--mode=gondor`) stripped of language meaning (`--mode=classical`, `--mode=gondor`)
- [ ] Allow custom Modes to be specified via a config language (Would TOML work?)
### Library
- [ ] Cirth
- [ ] English Mode?
- [ ] "Reverse" transcription; Tengwar to Latin text
- [ ] Switch all Tengwar codepoints to official Unicode values (when they are accepted)
### Output
### Tests
- [ ] Benchmark executable, if possible
- [ ] Test executable outputs

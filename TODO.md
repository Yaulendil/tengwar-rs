# TODO

---
## 1.2.0

### CLI
### Docs
### Input
- [ ] Allow custom Modes to be specified via a config language (Would TOML work?)
### Library
- [ ] Gondor "Full" Mode
### Output
### Tests

---
## TBD

### CLI
### Docs
### Input
- [ ] Definitively decide what to do wrt "language" vs "mode":
  - Would be very nice to have a dedicated `--language`/`-L` option for frontend clarity. How would this interact with having multiple Sindarin modes?
  - Possible division:
    - Explicit "mode" options (`--quenya`, `--gondor`) become "language" options (`--quenya`, `--sindarin`), each mapping to a "primary mode" for the language
    - Values of `--mode` option (`--mode=quenya`, `--mode=gondor`) stripped of language meaning (`--mode=classical`, `--mode=gondor`)
### Library
- [ ] Cirth
- [ ] English Mode?
- [ ] "Reverse" transcription; Tengwar to Latin text
- [ ] Switch all Tengwar codepoints to official Unicode values (when they are accepted)
### Output
### Tests
- [ ] Benchmark executable, if possible
- [ ] Test executable outputs

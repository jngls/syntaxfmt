# TODO

- [x] Write prefix and suffix in generated code
- [x] Clean up attributes (Common, Type, Field)
- [x] Change all explicit names for `ident` back in `parse_basic`
- [x] Conditionals
- [x] Else content
- [x] Invert logic and simplify eval and content matches
- [x] Explicit newline args and newline() api in SyntaxFormatter
- [x] syntax_else
- [x] compile failure on empty struct
- [ ] Need to build a HashSet of each generic type (state and element both with same lifetime creates a conflict)
- [ ] Clean up generics split in general
- [ ] Support OsString, OsStr, CString, CStr
- [ ] Convert everything to references
- [x] Refactor tests
- [ ] Update docs
- [ ] Merge
- [ ] Publish

# FUTURE

- [ ] Convert the whole library to use something similar to token stream
- [ ] Extract newlines as tokens
- [ ] Auto indent
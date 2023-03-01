# grrs

Based off of grrs from [Rust CLI Book](https://github.com/rust-cli/book).

---
# Table of Contents

<!-- TOC -->
* [grrs](#grrs)
* [Table of Contents](#table-of-contents)
* [What's Changed](#whats-changed)
<!-- TOC -->

---

# What's Changed
- Implemented BufReader instead of read_to_string for performance gains.
- Enumerated over lines to output the line number.
- Running total of pattern occurrence.
- A bit more error context with Anyhow, similar to how the book handles context.
  - [ ] Use ThisError or another error management crate for friendlier user-friendly error handling.
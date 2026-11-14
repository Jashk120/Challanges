# Challanges

Rust practice repo: my [Codeforces](https://codeforces.com/) solutions, written to learn Rust.

## Format

Every solution file (`<problem-id>.rs`, e.g. `339A.rs`) follows the same layout:

1. **First — the accepted solution.** The exact code I submitted that got Accepted.
2. **Second — the better version.** A commented-out rewrite at the bottom of the file using
   more idiomatic Rust (cleaner iteration, no manual index juggling, better stdlib usage).

Keeping both makes the progress visible: what I wrote to pass, and what I would write now.

## Running

```bash
rustc 339A.rs -o solution && ./solution < input.txt
```

or pipe the input straight in:

```bash
echo "1+3+2+1" | ./solution
```

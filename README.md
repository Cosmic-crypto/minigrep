# Minigrep
---
## Introduction

Minigrep is a useful, simple and fast copy of grep, a tool used for finding certain things in a file, built in rust
---

## Usage
```bash
minigrep <file> <query> [--findall/-f]
```
---

## Installation

For the best experiece I advice of installing rust and proceeding to enter these commands in your terminal:
```bash
cargo new minigrep
cd minigrep
# Go into src/main.rs change the code to the one provided in this repo
cargo intall --path .
where minigrep # Should output a path
```
this will install minigrep into your path so you can just call minigrep where ever you are from your terminal, now you can if you want uninstall rust with this command:
```bash
rustup self uninstall
```

Or just install the .exe provided in the releases and do 
```bash
./minigrep.exe <file> <query> [--findall/-f]
```
but you need be in the file that houses minigrep.exe
---

## Improvements
* add regex patterns
* add other flags
* add comments
---

This is my 2nd major rust project after the rc project, so it will be greatly appreciated if I can get any feed back

[![Build Status](https://app.travis-ci.com/Montana/yamashiro.svg?token=U865GtC2ptqX3Ezf3Fzb&branch=master)](https://app.travis-ci.com/Montana/yamashiro)

# Yamashiro 

![Yo](https://github.com/user-attachments/assets/310eb46d-c158-4f06-92f0-04f030bd0af3)

Yamashiro is a worker implementation in Rust, designed to process tasks sequentially for Travis CI.

## Features

- Create a worker with a unique ID
- Define tasks with IDs and durations
- Process tasks sequentially, simulating work with sleep durations

## Installation

Clone the repository:
```bash
git clone https://github.com/Montana/yamashiro.git
cd yamashiro
```

Build the project:
```bash
cargo build
```

Run the project with:
```bash
cargo run
```

Run tests with `cargo-pants`: 

```bash
 cargo pants --loud
```
To test `cargo-pants` run: 

```bash
cargo pants --pants_style JNCO
```
You should see something like this:

![Screenshot 2024-09-23 at 11 43 35 AM](https://github.com/user-attachments/assets/c1b33d1d-c339-4d1f-ad0f-ecac029b9e3a)

Once you do, enjoy using Yamashiro strategically in builds. 

### Author

>**Michael Mendy (2024) for Travis CI, GmbH.** 

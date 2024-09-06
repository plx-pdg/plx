<center>

![logo of PLX](imgs/logo.svg)
</center>

### **P**ractice programming exos in a delightful **L**earning e**X**perience

### Introduction

PLX (Practice Learning eXperience) is a project developed to enhance the learning of programming languages, with a focus on a smooth and optimized user experience. The goal of this project is to reduce the usual friction involved in completing coding exercises (such as manual compilation, testing, and result verification) by automating these steps.

PLX offers a text-based user interface (TUI) developed in Rust and supports multiple languages (C, C++, Java). It enables automatic compilation as soon as a file is saved, automated tests to compare program outputs, and instant display of errors and differences compared to a reference solution. The project draws inspiration from Rustlings and aims to create a more efficient learning experience, particularly for programming courses at HEIG-VD.### Installation

### Installation

#### Prerequisites

- [Cargo](https://www.rust-lang.org/tools/install)

Once you have cargo installed, simply run

```bash
cargo install plx
```

### Testing using our example folder

Once you have plx installed, you can try it on this repo's example folder

```bash
git clone git@github.com:plx-pdg/plx.git
cd plx/examples/full
plx
```

### Building from source

- Clone this repo

```bash
git clone git@github.com:plx-pdg/plx.git
```

- Build using `cargo`

```bash
cargo build --release
```

This will install all necessary dependencies and build the program

### Contributing

Please take a look at some of the issues [here](https://github.com/plx-pdg/plx/issues).

- Fork this project
- Create a branch
- Develop your solution
- Create a Pull Request


### License

We are currently waiting for our school's approval before applying an open source license.

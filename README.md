<center>

![logo of PLX](imgs/logo.svg)
</center>

### **P**ractice programming exos in a delightful **L**earning e**X**perience

### Introduction

PLX is a project developed to enhance the learning of programming languages, with a focus on a smooth and optimized learning experience. The goal of this project is to reduce the usual friction involved in completing coding exercises (such as manual compilation, running, testing, and result verification) by automating these steps.

PLX offers a terminal user interface (TUI) developed in Rust and supports multiple languages (currently C and C++). It enables automatic compilation as soon as a file is saved, automated checks to compare program outputs, and instant display of errors and output differences. The solution code can also be displayed. The project draws inspiration from [Rustlings](https://rustlings.cool/) and aims to create a more efficient learning experience, particularly for programming courses at HEIG-VD.

### Installation

#### Prerequisites

- [Cargo](https://www.rust-lang.org/tools/install)

Once you have cargo installed, simply run

```bash
cargo install plx
```

### Testing using our example folder

Once you have plx installed, you can try it on this repo's example folder

> [!IMPORTANT] 
> Set the $EDITOR environment variable if you wish for your editor to be opened when starting an exo

> [!WARNING] 
> The open editor feature is currently unstable, using a terminal based editor causes problems
> The following editors were tested and work fine: `code`, `idea` and `codium`

> [!IMPORTANT] 
> Only C and C++ exercises are valid for now, java and other languages support is comming soon™

```bash
git clone git@github.com:plx-pdg/plx.git
cd plx/examples/full
plx
```

On Linux and MacOS, you can easily change `EDITOR` just for PLX, here is an example for `VSCode`.
```sh
EDITOR=code plx
```

The useful shortcuts are defined under shortcut `?`, you can mostly type `l`, `j` or `k` to move up and down, until you you reach the exo, then the editor should open and then you can do the exo (or just fill the solution next to it see `.sol.c` file), save and see changes...

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

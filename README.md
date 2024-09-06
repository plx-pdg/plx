<center>

![logo of PLX](imgs/logo.svg)
</center>

### **P**ractice programming exos in a delightful **L**earning e**X**perience

### Introduction

PLX (Practice Learning eXperience) est un projet développé pour améliorer l'apprentissage des langages de programmation, avec un focus sur une expérience utilisateur fluide et optimisée. Ce projet vise à réduire les frictions habituelles liées à la réalisation d'exercices de code (comme la compilation manuelle, les tests et la vérification des résultats) en automatisant ces étapes.

PLX propose une interface utilisateur en texte (TUI) développée en Rust, et supporte plusieurs langages (C, C++, Java). Il permet une compilation automatique dès qu'un fichier est sauvegardé, des tests automatisés pour comparer l'output des programmes, et un affichage instantané des erreurs et différences par rapport à une solution de référence. Le projet prend inspiration de Rustlings et vise à créer une expérience d'apprentissage plus efficace, notamment pour les cours de programmation à la HEIG-VD.


### Installation

#### Prerequisites

- [Cargo](https://www.rust-lang.org/tools/install)

Once you have cargo installed, simply run

```bash
cargo install plx
```


### Usage

Open your terminal in a plx folder, for instance this [example folder](./examples/full) and simply run

```bash
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

Pour l'instant, nous attendons l'approbation de l'école pour pouvoir ajouter une license libre.

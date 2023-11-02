# 🦀 ***My Journey*** Getting _Started_ With Rust

This repository is a collection of projects I am working on while learning Rust.

## 📚 Table of Contents

The root directory is a Cargo workspace, with each subdirectory being a separate crate.

- `datastructures`: Implementations of common data structures. (Mostly not optimized, just to get a feel for the language)
    - `linked-list`: A basic linked list implementation.
    - `binary-search-tree`: A basic binary search tree implementation.
    - `vector`: A dynamic array implementation.

- `todo-app`: A basic todo app using the `crossterm` crate. (WIP)

### 🔥 Wishlist

This in the most part is a wishlist of things I want to do with Rust.

I will be adding to this list as I go along while hopefully crossing things off as I go. 🫡

- `datastructures`:

    - `stack`: A stack LIFO implementation.
    - `queue`: A queue FIFO implementation.
    - `hash-table`: A basic hash table implementation.
    - `graph`: A graph implementation.

- `bevy-tetris`: A tetris clone using the `bevy` crate.
- `webserver`: A basic webserver using the `axum` crate.
- `leetcode`: Solutions to LeetCode problems.
- `advent-of-code`: Solutions to Advent of Code problems.

- `virtual-machine-ecosystem`: A virtual machine ecosystem based on a very simple ISA.

    - **NOTE** This is a long term project. The project will be called `YawningEcosystem` 🥱  

    - `ysm`: An assembler.
        - `parser`: A parser for the assembler using the `nom` crate.

    - `yvm`: The virtual machine.

    - `yawn`: A high level language that compiles to the virtual machine's assembly.

        - `yawnc`: A compiler for the high level language.

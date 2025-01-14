# Celui – A Minimalistic, Modular UI Library 🌱

> Written in Rust, btw 🦀

Celui (short for **Celery UI**, and pronounced playfully as _"Celewy"_ :D) is a highly modular, zero-dependency UI library designed to be minimalistic and versatile. Its main goal is to serve as the foundational layer for any project that needs a **graphical user interface**. Each component is decoupled, allowing maximum flexibility and minimal footprint. ♻️

##### Here are some of the bold goals this library strives for:

- **Minimalistic:** A library with zero dependencies, not even on the Rust standard library.
- **Modular:** Completely decoupled components for rendering, input handling, and more.
- **Versatile:** Ideal for building applications, game engines, or even custom operating systems!

> While these goals shape the project's direction, _Celui_ is still in its earliest stages! 💡

## Modules 📦

To lay the groundwork for these goals, the workspace is organized into multiple modules:

- **[celui_core](/tree/master/modules/celui_core)**: The heart of the system. Provides essential abstractions, utilities, and the core foundation.
- **[celui_renderer](/tree/master/modules/celui_renderer)**: Handles the rendering of primitives like triangles, rectangles, circles, and text.
- **[dev](/tree/master/modules/dev)**: A binary module used for experimenting with the library and testing in general.

You can use the modules individually or combine them based on your project's needs. However, the core module contains most of what'll you need. 🧩

## Getting Started 🏁

Since this project is just beginning, there isn't any ready-to-use code yet. However, you can still set up the workspace and follow along as development progresses:

**Clone the Repo**

```bash
git clone https://github.com/IrregularCelery/celui.git
cd celui
```

## License 📜

**Celui** is released under the [MIT License](/blob/master/LICENSE).

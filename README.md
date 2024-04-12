

## SeraphyBR's Blog

This is a rewrite of my previous blog built with Jekyll, but this time I'm diving into the world of Front-End Single Page Applications (SPAs) using Rust and the Leptos framework.

**Prerequisites:**

- **Node.js:** You will need to have Node.js installed on your system. Node.js can be downloaded and installed from the official website: [https://nodejs.org/en/download](https://nodejs.org/en/download)
- **Git:** You will need to have Git installed on your system. Git can be downloaded and installed from the official website: [https://git-scm.com/downloads](https://git-scm.com/downloads)
- **Rust Toolchain:** You will need to have the Rust toolchain installed on your system, with Rust nightly and Wasm target.

**Setting Up SeraphyBR's Blog:**

1. **Clone the repository:**

```sh
git clone https://github.com/SeraphyBR/seraphybr.github.io seraphybr-blog
```

2. **Navigate to the project directory:**

```sh
cd seraphybr-blog
```

**Notes:**

- This project uses [Trunk](Trunk) to compile and serve the application during development. The `Trunk.toml` file configures the development options, so parameters are not needed in the `trunk serve` command.
- This blog uses Tailwind CSS for styling and the Tailwind Typography plugin to enhance typography. The installation and configuration of Tailwind CSS and the plugin are beyond the scope of this readme, but resources can be found online.
- This blog uses a Rust crate to convert markdown files into HTML.

**Development:**

To start developing the blog, run the following command:

```sh
trunk serve
```

This will start a development server and open my blog at http://localhost:3000.

**Additional Details:**

For more information about the development experience, decisions made, and other details, please visit the Projects page on the blog itself.

[Leptos]: https://github.com/leptos-rs/leptos

[Trunk]: https://github.com/trunk-rs/trunk
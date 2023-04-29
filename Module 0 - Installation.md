This module exists to get off the ground with rust.

# Installing 

To install rust, you can use whatever mechanism you want, but the official route is to use rustup.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You can then use `rustup update` to update your installation. You can also install new components. For more information, the [official documentation](https://www.rust-lang.org/learn/get-started) is a great place to start. 

# Picking an Editor

Rust has two primary ways of getting code intelligence. 

One is to use `IntelliJ Rust`  plugin via CLion, or another IntelliJ IDE. While I don't personally use it, it is apparently very good. If you already have a licence for IntelliJ and use it, this is probably your best bet.

Option two is to use `rust-analyzer` via a Language Server Protocol plugin. VSCode has native LSP support (they invented the standard). VIM has [plugins](https://rust-analyzer.github.io/manual.html#vimneovim), emacs has eglot and ls-mode combined with rustic (emacs is more complicated than most -- if you _actually_ want to use emacs, I can help). Those options will likely make everyone happy.

If you want to live more on the edge, the folks behind Atom and Tree-Sitter are writing a new IDE (in Rust, of course) called [zed](https://zed.dev/). It only runs on Mac. It's proprietary software.

If you like open source software but emacs, vim and VS Code are too slow for you, try out [lapce](https://lapce.dev/). Lapce has a terrible name (seriously) but is _really_ fast. It also has support for go, rust, ruby, php and ts via plugins. The interface is similar to VSCode, but its native code with immediate mode text rendering, which means you feel like you write faster... even if you don't. It is, however, very alpha. I daily drive it, but it isn't a complete polished package, yet.

# Cargo

Cargo is rust's native package manager. Its _very_ good (a lot of people seriously claim they like rust just because of cargo). Packages in rust are called crates, and the primary public repository of those is [Crates.io](https://crates.io). 

We'll get to using cargo in the first (and of course subsequent) modules. For now, let's create an empty project.

Navigate to the directory where you want to create your rust project, and then just type:

```
cargo new rust-training
```

This will create a new folder with the structure you need for a binary rust application. For more information on how to use cargo, see the official [docs](https://doc.rust-lang.org/cargo/index.html).

# Reading This Training

The modules herein are all in Markdown format, and specifically the format respected by [Obsidian](https://obsidian.md/). You may use whatever markdown renderer you wish, including just Github's UI, but I recommend Obsidian as it is easy to use, and has a lot of power.

Most of the examples in this training are executable. I also have a home-grown literate programming tool called [Betwixt](https://github.com/qmuloadmin/betwixt) which you can install if you wish to execute the examples. This is still very much unstable, early software, so the API may change, but its a whole lot more portable than the old format I used for literate programming -- Emacs org mode.

As an example, to execute the source block below:
<?btxt+rust filename='main.rs' cmd='rustc main.rs && ./main' mode='overwrite'  tag='example' ?>
```rust ex1
fn main() {
	println!("hello, betwixt!");
}
```

You should install betwixt, then type

```
betwixt -t example -e ex1 -o /tmp 'Module 0 - Installation.md'
```

More documentation is available on the Betwixt github.
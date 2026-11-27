//! spi3derr: spi3der in Rust, one program for the terminal, the browser and the desktop.
//!
//! This release only reserves the name. The first real iteration will be the winner of the spid3err contest: two
//! dependency-free Rust ports of spider-paint's terminal version (v26.11.26003), written by Claude Opus 5.5.

const MESSAGE: &str = "spi3derr — spi3der in Rust (terminal, browser, desktop)

This release only reserves the name.
The first real iteration will be the winner of the spid3err contest:
two dependency-free Rust ports of spider-paint's terminal version.

  https://github.com/Pengeszikra/spi3derr
  Tüskék és Gyökerek: https://tuskek-gyokerek.petervivo.chatgpt.site/";

fn main() {
    if std::env::args().any(|a| a == "-V" || a == "--version") {
        println!("spi3derr {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    println!("{MESSAGE}");
}

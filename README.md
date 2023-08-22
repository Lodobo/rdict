# rdict
An offline CLI dictionary written in rust, using data from wiktionary.

## Installation

### From source:

#### Clone the Repository:
```bash
$ git clone https://github.com/Lodobo/rdict
$ cd rdict
```

####  Install the Rust toolchain: [https://rustup.rs](https://rustup.rs)

#### Build and install:
```bash
$ cargo build --release
```

#### Download json (1.5GB):
```bash
# This will download a jsonl file to ~/.local/share/rdict
$ ./target/releae/rdict download-json
```

#### Create sqlite database:
```bash
# This will create a .db file to ~/.local/share/rdict
$ ./target/release/rdict json-to-db
```

## Usage of rdict:
|Options|Description|
|----|----|
|-w [WORD]|Search word|
|--help|Print help|
|-V|Print version|

## See also
- Tatu Ylonen's [Wiktextract](https://github.com/tatuylonen/wiktextract): A utility for extracting data from wiktionary. The lexical data used by gdict comes from dumps provided by Ylonen on [kaikki.org](https://kaikki.org/)
    > [Wiktextract: Wiktionary as Machine-Readable Structured Data](http://www.lrec-conf.org/proceedings/lrec2022/pdf/2022.lrec-1.140.pdf), Proceedings of the 13th Conference on Language Resources and Evaluation (LREC), pp. 1317-1325, Marseille, 20-25 June 2022.
- [gdict](https://github.com/Lodobo/gdict): Another version of this software written in go.

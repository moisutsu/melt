<h1 align="center">Welcome to melt 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/crates/v/melt.svg" />
  <a href="https://github.com/moisutsu/melt/blob/master/LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
  <a href="https://twitter.com/moisutsu" target="_blank">
    <img alt="Twitter: moisutsu" src="https://img.shields.io/twitter/follow/moisutsu.svg?style=social" />
  </a>
</p>

> This is a command-line tool to decompress files of various extensions

The supported extensions are as follows.

| Extension       | Compression Format | Decompression tool |
| --------------- | ------------------ | ------------------ |
| zip             | zip                | unzip              |
| gz              | gzip               | gunzip             |
| Z               | compress           | uncompress         |
| bz2             | bzip2              | bunzip2            |
| tar             | tar                | tar                |
| tar.gz<br>tgz   | tar + gzip         | tar                |
| tar.Z<br>taz    | tar + compress     | tar                |
| tar.bz2<br>tbz2 | tar + bzip2        | tar                |
| tar.xz          | tar + xz           | tar                |
## Install

```sh
cargo install melt
```

## Usage

It automatically recognizes the file extension and calls the decompression tool internally to decompress the file.

```sh
melt sample.zip
melt sample.gz
melt sample.tar.gz
```

## Author

👤 **moisutsu**

* Twitter: [@moisutsu](https://twitter.com/moisutsu)
* Github: [@moisutsu](https://github.com/moisutsu)

## Show your support

Give a ⭐️ if this project helped you!

## 📝 License

Copyright © 2020 [moisutsu](https://github.com/moisutsu).<br />
This project is [MIT](https://github.com/moisutsu/melt/blob/master/LICENSE) licensed.

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_

# fortune_telling_lot

An Omikuji (fortune-telling lot) Linux kernel module in Rust

Ubuntu 23.10

```
$ sudo apt install rustc-1.68 rust-1.68-src rustfmt-1.68 bindgen-0.56 \
    llvm clang build-essential linux-lib-rust-$(uname -r)
```

```
$ git clone https://github.com/mnogu/fortune_telling_lot.git
$ cd fortune_telling_lot
$ make
```

```
$ sudo update-secureboot-policy --new-key
$ sudo make install
```

```
$ sudo modprobe fortune
$ journalctl -k -n 1
Mar 07 12:34:56 foo-server kernel: fortune: 大吉 (Great blessing)
```

```
$ sudo modprobe -r fortune
$ sudo modprobe fortune
$ journalctl -k -n 1
```

## Reference

* https://gihyo.jp/admin/serial/01/ubuntu-recipe/0793 (Japanese)
* https://en.wikipedia.org/wiki/O-mikuji
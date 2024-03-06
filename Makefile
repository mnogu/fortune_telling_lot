# SPDX-License-Identifier: CC0-1.0

KVER ?= $(shell uname -r)
KDIR ?= /usr/lib/modules/$(KVER)/build

RUSTFMT = /usr/lib/rust-1.68/bin/rustfmt
RUST_FLAGS = CROSS_COMPILE=x86_64-linux-gnu-
RUST_FLAGS += HOSTRUSTC=rustc-1.68
RUST_FLAGS += RUSTC=rustc-1.68
RUST_FLAGS += BINDGEN=bindgen-0.56
RUST_FLAGS += RUSTFMT=$(RUSTFMT)
RUST_FLAGS += RUST_LIB_SRC=/usr/src/rustc-1.68.2/library

default:
	$(MAKE) $(RUST_FLAGS) -C $(KDIR) M=$$PWD/src

install: default
	kmodsign sha512 \
                /var/lib/shim-signed/mok/MOK.priv \
                /var/lib/shim-signed/mok/MOK.der \
                src/fortune.ko
	$(MAKE) -C $(KDIR) M=$$PWD/src modules_install
	depmod -A

fmt:
	find . -type f -name '*.rs' | xargs $(RUSTFMT)

clean:
	$(MAKE) $(RUNST_FLAGS) -C $(KDIR) M=$$PWD/src clean

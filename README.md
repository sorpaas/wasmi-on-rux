# `wasmi` on Rux

This `no_std` version of `wasmi` allows you to run it on the [Rux
microkernel](https://source.that.world/source/rux/).

## Get Started

To build and run `wasmi_nostd`, it needs to be bundled with the Rux
microkernel and be executed together. First, initialize the submodules
which points to a revision of
[Rux](https://source.that.world/source/rux/).

```lang=bash
git submodule update --recursive --init
```

After that, install [Nix](https://nixos.org/nix/). Then run:

```lang=bash
nix-shell
```

This will install all the dependencies you need to build the kernel
and `wasmi` in a specialized Nix location without any modification to
your actual system.

After that, to run the kernel with `wasmi` init program, run:

```lang=bash
make sinit-wasmi
```

Under the hook, the script will start a QEMU virtual machine,
bootstrap the kernel, which will then load the init program and invoke
a simple WebAssembly module.

## Bug Reports and Contributing

For bug reports and pull requests related to Rux, please use this
[Phabricator instance](https://source.that.world/). Bug reports can be
submitted to [Maniphest](https://source.that.world/maniphest/) and
refer to [Rux](https://source.that.world/source/rux/)'s contributing
guide for sending pull requests.

For bug reports and pull requests related to `wasmi`, please use
`wasmi`'s [issue tracker](https://github.com/paritytech/wasmi).

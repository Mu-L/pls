---
lang: en-GB
title: Get started
description: >-
  Installing pls is as easy as 'pip install --user pls', provided you already
  have the prerequisites.
---

# Get started

Thanks for using `pls`!

## Prerequisites

`pls`, being a Python package needs the following:

- Python ≥ 3.8
- `pip`

You see, `pls` has very few needs.

## Installing

### `pip`

To install `pls`, install it as you would any other Python package.

```:no-line-numbers
$ pip install --user pls
```

### `pipx`

You can also install `pls` via [`pipx`](https://pypa.github.io/pipx/).

```:no-line-numbers
$ pipx install pls
```

## Updating

To update `pls` when you've fallen behind the latest version, run the following
command.

```:no-line-numbers
$ pip install --user --upgrade pls
```

If you used any alternative methods to install `pls`, follow the documentation
for the respective tool to update packages.

## Verifying

To check if `pls` is installed and discoverable in your path, run the following
command. If you see the same version number as on
[PyPI](https://pypi.org/project/pls/), you're all set!

```:no-line-numbers
$ pls -v
pls x.y.z
```

## Using

To run `pls`, type the command into any terminal.

```:no-line-numbers
$ pls
```

To get help, run `pls` with the `--help`/`-h` flag or _read this documentation_!

```:no-line-numbers
$ pls --help
```

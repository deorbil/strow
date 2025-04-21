<!-- markdownlint-disable MD013 -->

# strow

strow is a symlink manager that make files located in separate directories appear to be installed in the same place.

## How It Works

It recursively match all files in the package directory and the target directory, then tries to create symlink if target doesn't exists, otherwise it will continue to recurse directory until all entries are processed.

## Usage

```bash
strow [OPTIONS] [PACKAGE]...
```

## Features

- [x] Basic stow functionality
- [x] Multi package
- [ ] Delete and restow
- [ ] Everthing else in `stow`

## Credits

strow is based on [GNU Stow][stow]

For further information, please refer to [GNU Stow documentation][stow-docs].

[stow]: https://www.gnu.org/software/stow/
[stow-docs]: https://www.gnu.org/software/stow/manual/stow.html

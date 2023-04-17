# Convert Beginning Spaces or Tabs

Convert either beginning spaces to tabs, or tabs to spaces.

The default number of spaces is 4.

Build

```shell
cargo build --release
```
---

Usage

```shell
convert_beginning_whitespaces_rust --ws-from space --comment-char "*" --num-spaces 4

# if you need help
convert_beginning_whitespaces_rust --help
```

Help

```shell
Convert beginning whitespaces from space to tab, or vice versa

Usage: convert_beginning_whitespaces_rust [OPTIONS] --ws-from <WS_FROM> [FILES]...

Arguments:
  [FILES]...  Files to operate on

Options:
  -n, --num-spaces <NUM_SPACES>      Number of spaces to convert into / from [default: 4]
  -w, --ws-from <WS_FROM>            Convert from spaces or from tabs [possible values: space, tab]
  -c, --comment-char <COMMENT_CHAR>  Optional: character that starts a multi-line comment
  -h, --help                         Print help
  -V, --version                      Print version
```

## gcwd
Submit your Git commit with sepcific date.

```bash
# Basic
gcwd 13:37
# With message
gcwd -m 'Hello World!' '13:37'
# Signed commit
gcwd -s '13:37'
```

## CLI
```bash
Usage: gcwd [OPTIONS] [TIME]

Arguments:
  [TIME]  

Options:
  -m, --message <MESSAGE>  Commit message
  -s, --sign               Sign commit
  -h, --help               Print help
  -V, --version            Print version
```

Valid `TIME` formats:
- `HH:MM` or `HH:MM:SS`
- `HH-MM` or `HH-MM-SS`
- `YYYY-MM-DD HH:MM:SS`

If `YMD` is not provided, then `YYYY-MM-DD` will be set to the current date.
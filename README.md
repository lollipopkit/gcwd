## gcwd
Submit your Git commit with sepcific date.

```bash
# Basic
gcwd 13:37
# With message
gcwd '13:37' -m 'Hello World!'
```

## CLI
```bash
Usage: gcwd [OPTIONS] [TIME]

Arguments:
  [TIME]  

Options:
  -u, --update             Check gcwd update
  -m, --message <MESSAGE>  Commit message
  -h, --help               Print help
  -V, --version            Print version
```

Valid `TIME` formats:
- `HH:MM` or `HH:MM:SS`
- `HH-MM` or `HH-MM-SS`
- `YYYY-MM-DD HH:MM:SS`

If `YMD` is not provided, then `YYYY-MM-DD` will be set to the current date.
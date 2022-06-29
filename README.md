# chromoly

```
chromoly 0.1.0
   ▄                                             ▄
    █▄▄▀█     ▄    ▄▄▄▄▄     ▄    ▄▄▄       ▀▄    █
   ▄▀   █      █▀▀▀    █      ▀▀█▀           █    █
  ▄▀   █       ▀▄     ▄▀     ▀▄▄█▄▄▀▀▀▀      █    █
     ▄▀         █ ▄▄▄▄█         █                █
   ▄▀           ▀▀     ▀         ▀▀▀▀▀         ▄▀

chrome protector

USAGE:
    chromoly [OPTIONS] --config <CONFIG>

OPTIONS:
    -c, --config <CONFIG>    config file
    -h, --help               Print help information
        --no-daemonize       not daemonize
    -V, --version            Print version information
```

## config format

```yaml
pid_file: /path/to/daemon/pid
log_file: /path/to/daemon/log
user: daemon_user
workdir: /path/to/daemon/workdir
chrome_binary: # optional
 - /path/to/chrome/binary
 - /path/to/other/binary
cookies: # optional
 - /path/to/cookie/file

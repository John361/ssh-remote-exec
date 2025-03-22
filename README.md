# SSH Remote Exec
Execute remote ssh commands on multiple hosts

# Usage
```
Usage: ssh-remote-exec --hosts <HOSTS> --username <USERNAME> --public-key <PUBLIC_KEY> --private-key <PRIVATE_KEY>

Options:
  -H, --hosts <HOSTS>              Required - Hosts
  -U, --username <USERNAME>        Required - Username
  -P, --public-key <PUBLIC_KEY>    Required - Public key
  -K, --private-key <PRIVATE_KEY>  Required - Private key
  -h, --help                       Print help
  -V, --version                    Print version
```

Hosts may be specified multiple time when working with multiple hosts

Example:
```shell
ssh-remote-exec -H 192.168.132.133:22 -H 192.168.132.133:22 -U root -P tmp/id_ed25519.pub -K tmp/id_ed25519
```

# SSH Remote Exec
Execute remote ssh commands on multiple hosts

# Usage
- Hosts may be specified multiple time when working with multiple hosts
- Public key must be at the same place of the private key and have .pub extension

```
Usage: ssh-remote-exec [OPTIONS] --hosts <HOSTS> --username <USERNAME> --identity <IDENTITY> --command <COMMAND>

Options:
  -H, --hosts <HOSTS>        Required - Hosts
  -U, --username <USERNAME>  Required - Username
  -I, --identity <IDENTITY>  Required - Identity file (Private key)
  -C, --command <COMMAND>    Required - Command
  -P, --password <PASSWORD>  Optional - Password [default: ]
  -h, --help                 Print help
  -V, --version              Print version
```

Example:
```shell
ssh-remote-exec -H 192.168.132.133:22 -H 192.168.132.133:22 -U root -I tmp/id_ed25519 -C "ls .bashrc"
ssh-remote-exec -H 192.168.132.133:22 -H 192.168.132.133:22 -U root -I tmp/id_ed25519 -C "sudo apt update" -P changeme
```

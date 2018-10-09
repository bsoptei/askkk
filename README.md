# askkk
Simple REPL for communication with a TCP server using the console

## Usage
```sh
cargo run --release [tcp address]
```

## Example: connect to and communicate with [kavakava](https://github.com/bsoptei/kavakava)

```p
askkk:
update John Doe;42;
OK
askkk:
update Big Oak Tree;420;
OK
askkk:
bykeys John Doe;
{"John Doe": "42"}
askkk:
byvals 420;
{"Big Oak Tree": "420"}
askkk:
```

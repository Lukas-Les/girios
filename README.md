# girios
Girios is a tree based database, which I'm writing for learning purposes

## How to run

To run girios daemon:
```
cargo run --bin open-server
```

And to use it:
You can use it in two ways.
First, is telnet. If you on mac, and don't have telnet installed, run brew install telnet.
```
telnet localhost 42069
```
Second, you can interact with daemon through a client.
To run client:
```
cargo bin --open-client
```


To generate and open documentation:

```
cargo docs --no-deps --open
```

## How to use

available methods:
```
insert <path> <value>
get <path>
hit <path>
delete <path>
```

e.g.
```
insert foo bar
```
will insert value bar into path foo

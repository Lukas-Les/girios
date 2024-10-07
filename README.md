# girios
Girios is a tree based database, which I'm writing for learning purposes

## How to run

To run girios daemon:
```
cargo run --bin server
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
cargo bin --cli-client
```


To generate and open documentation:

```
cargo docs --no-deps --open
```

## How to use

Common commands
```
create <structure type> <name>
destroy <structure type> <name>
```

Ctree commands
```
ctree <name> insert <key> <value>
ctree <name> get <key>
ctree <name> hit <key>
ctree <name> delete <key>
ctree <name> scan
```

e.g.
```
create ctree my_tree
ctree my_tree insert foo bar
```
will create char tree named my_tree and insert value bar into path foo


## [Roadmap](https://docs.google.com/spreadsheets/d/1rAe194TiP8Uh3TWq-6t2CMmyK_q8IUbezFoHdePunWQ/edit?usp=sharing)

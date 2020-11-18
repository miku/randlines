# randlines

Print out random number of lines from a line oriented file.

```shell
$ randlines
usage: randlines -n NUM FILE

Emit a random subset of lines from a file. This is a probabilistic program, you will not get
exactly `n` lines, but about `n` lines.

🈹 Typically, you can use shuf(1) which uses reservoir sampling and is very efficient. However, if we
want to extract 10M random lines from a file of 100M lines, shuf(1) might be killed.
```

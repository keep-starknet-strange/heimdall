# Infra

## Cairo

### Usage

Compile cairo file:

```sh
# $FILE must be the path to the cairo file
docker run --rm -v "$PWD":"$PWD" -w "$PWD" starkrocket/cairo-cli cairo-compile $FILE 
```

Run cairo file

```sh
# $FILE must be the path to the cairo file
docker run --rm -v "$PWD":"$PWD" -w "$PWD" starkrocket/cairo-cli cairo-run -p $FILE 
```

You can also alias these like so:

```sh
alias cairo1='docker run --rm -v "$PWD":"$PWD" -w "$PWD" starkrocket/cairo-cli'
```

and usage would be

```sh
# $FILE must be the path to the cairo file
cairo1 cairo-compile cairo-compile $FILE 
cairo1 cairo-run -p $FILE 
```

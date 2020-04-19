# changers

## Usage

#### Create artifacts

```sh
changers create bugfix "Fixed some Error that caused issues" --for=user
```

This will create a changelog artifact of type `bugfix` in the directory `./changelog/unreleased` as a single file.

#### Release a Changelog

```sh
changers release 0.1.0
```

This will rename the current `unreleased` subdirectory into `v0.1.0`

#### Aggregate a CHangelog

```sh
changers render 0.1.0 --type=markdown --for=user
```

This will aggregate a changelog with `markdown` as the target format

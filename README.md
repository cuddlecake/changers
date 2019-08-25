# changers

A command-line tool for creating, aggregating and publishing changelog artifacts

## Usage

#### Create artifacts

```sh
changers create bugfix "Fixed some Error that caused issues"
```

This will create a changelog artifact of type `bugfix` in the directory `./changelog/unreleased` as a single file.

#### Release a Changelog

```sh
changers release "v0.1.0"
```

This will rename the current `unreleased` subdirectory into `v0.1.0`

#### Aggregate a CHangelog

```sh
changers aggregate "v0.1.0" --type=markdown
```

This will aggregate a changelog with `markdown` as the target format

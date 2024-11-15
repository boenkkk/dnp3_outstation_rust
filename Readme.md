# DNP3 Outstation Simulator

DNP3 Outstation Simulator

## Getting Started

### Dependencies

- Rust

### Executing program

- Run

```
cargo run
```

- Build

```
cargo build --release
```

## Conventional Commit

Do Before Commit:

- Check code convention & file naming
- Remove unused code, imports, or variable
- Check warning
- Add TODO if code isnt finished
- Keep code to clean

### New Feature or Update Feature

```
[new|patch]{file name or feautre name}
-description
```

example:

```
[new]httputil
-method postReq exception cant throws

[patch]httputil
-method postReq exception now can throws
-add method getReq

[new]measure analog input
-method handleAnalogInput cant call httputil

[patch]measure analog input
-method handleAnalogInput now can call httputil
-add method handleBinaryInput
```

## Authors

- [Budi Santoso](https://blog.boenkkk.dev/)

## Acknowledgments

- [README-Template.md](https://gist.github.com/DomPizzie/7a5ff55ffa9081f2de27c315f5018afc)

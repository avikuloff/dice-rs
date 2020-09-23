# dice-rs
Very simple library for rolling dice.

## Installation
Add this to your `Cargo.toml`
```toml
[dependencies]
roll-dice = "0.2"
```

## Usage
Roll `3d6` (3 six-sided die) and get sum of the results
```
let rolled = roll(3, NonZeroU32::new(6).unwrap());
let sum: u32 = rolled.iter().sum();
```
Roll one `d20`
```
let d20 = Die::new(NonZeroU32::new(20).unwrap());

d20.roll();
```

## License
[MIT license](LICENSE)

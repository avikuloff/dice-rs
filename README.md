# dice-rs
A library for rolling dice.

## Installation
Add this to your `Cargo.toml`
```toml
[dependencies]
roll-dice = "0.1"
```

## Usage
Roll `3d6` (3 six-sided die) and get the sum of the results
```
use roll_dice::roll;

let rolled = roll(3, 6);
let sum: u32 = rolled.iter().sum();
```

## License
[MIT license](LICENSE)

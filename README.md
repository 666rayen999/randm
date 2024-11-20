# Randm

**Randm** is a simple, fast, and efficient random generation crate for Rust. It aims to provide minimal overhead, quick random generation, and a small memory footprint, making it ideal for lightweight applications or performance-critical tasks.

### Features

- **High Performance:** Uses bitwise operations to generate random numbers quickly.
- **Small and Efficient:** Minimal memory usage, focusing on speed and efficiency.
- **Easy to Use:** Simple API for generating random numbers with no dependencies.

### Installation

```sh
cargo add randm
```

### Usage

```rust
use randm::*;

fn main() {
    let mut rng = Random::new(); // seed is set to time::now()
    let random_number: u32 = rng.get();
    println!("Generated random number: {}", random_number);

    let mut rng = Random::seed(666); // or you can set seed manually
    let random_number: f32 = rng.get();
    println!("Generated random number: {}", random_number);
}
```

### Types

**RandomT** trait is already implemented for these types:
- **bool, char**
- **u8, u16, u32, u64, u128**
- **i8, i16, i32, i64, i128**
- **isize, usize**
- **f32, f64:** number between 0.0 and 1.0
- **tuple, array:** tuple max 12 elements

### Custom Type

```rust
use randm::*;

#[Debug]
struct Vec2 {
  x: f32,
  y: f32,
}

impl RandomT for Vec2 {
  fn random(r: &mut Random) -> Self {
    Self {
      x: r.get(),
      y: r.get(),
    }
  }
}

fn main() {
    let mut rng = Random::new();
    let vec2: Vec2 = rng.get();
    println!("Generated vec2: {:?}", vec2);
}
```

### How It Works

it uses the Xorshift algorithm with a period of `2^64-1`, meaning it will produce a repeated sequence only after `2^64-1` generations, or `18,446,744,073,709,551,615` unique values.

this is the algorithm used:
```c
x ^= x << 7;
x ^= x >> 9;
```

[Read More](https://en.wikipedia.org/wiki/Xorshift)

### Planning

macro feature to make it even simpler for custom structs

```rust
use randm::*;

#[Debug, Random]
struct Vec2 {
  x: f32,
  y: f32,
}

fn main() {
    let mut rng = Random::new();
    let vec2: Vec2 = rng.get();
    println!("Generated vec2: {:?}", vec2);
}
```

### Contributing:

Feel free to open issues or submit pull requests to improve the format.

### License

This project is licensed under the MIT License.

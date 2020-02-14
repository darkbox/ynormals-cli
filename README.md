# ynormals-cli
Simple command line tool to flip image channels written in rust

## Usage
This flips channels red (r) green (g) and blue (b). Posible values are **[r|g|b|a]**:
```
./ynormals filename --flip rgb
```

To flip just the green channel for example:
```
./ynormals filename --flip g
```
To access help, type:
```
./ynormals --help
```

## Build
```
cargo build --release
```

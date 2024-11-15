# Xascii
Simple tools for image to ascii
xascii is simple tools that can convert image files into ascii
### Features
- Fast image processing written in Rust
- Supports multiple image formats (PNG, JPG, JPEG, GIF)
- Customizable ASCII character sets
- Preserves aspect ratio
- Terminal-friendly output

## Get-startto!
### Clone the repository
```bash
git clone https://github.com/noraainuse/xascii
cd xascii
```
### Build and install
```bash
cargo build --release
cargo install --path .
```

# System-specific Dependencies
### Windows
- Install Visual Studio Build Tools
- Install Git for Windows
### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install build-essential pkg-config
```
### Fedora
```bash
sudo dnf groupinstall "Development Tools"
```
### Arch Linux
```bash
sudo pacman -S base-devel
```

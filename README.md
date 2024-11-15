# Xascii
Simple tools Convert both image and video to ascii ヽ(°〇°)ﾉ
- xascii is simple tools that can convert image files into ascii  (*￣▽￣)b
![Alt text](swappy-20241115-190010.png)

### Features
- Fast image and video processing written in Rust
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
```
### Install rustup (Rust toolchain manager) | Skip if you have installed
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Verify installation
```bash
rustc --version
cargo --version
```
## System-specific Dependencies
### Windows
- Install Visual Studio Build Tools
- Install Git for Windows
### Linux 
- Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install build-essential pkg-config
```
- Arch linux
```bash
sudo pacman -S base-devel
```
- Fedora
```bash
sudo dnf groupinstall "Development Tools"
```
### macOS
```bash
xcode-select --install
```
# Troubleshooting
If you encounter any issues:
- Ensure your Rust toolchain is up to date:
```bash
rustup update
```
- Clear Cargo cache:
```bash
cargo clean
```
- Check system dependencies
- Try rebuilding with verbose output:
```bash
cargo build -v
```

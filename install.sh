#!/bin/bash
echo "🌱 Cài đặt TriOS..."
pkg update -y && pkg upgrade -y
pkg install -y rust git make binutils
git clone https://github.com/Thanhdlpb/TriOS.git ~/TriOS
cd ~/TriOS
cargo build --release
echo 'export PATH="$HOME/TriOS/target/release:$PATH"' >> ~/.bashrc
source ~/.bashrc
echo "✅ TriOS đã được cài đặt thành công!"
echo "👉 Chạy REPL: tri chay"
echo "👉 Chạy VM: trivm file.tbc"
echo "👉 Chạy Runtime: trios"

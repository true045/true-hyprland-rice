#!/bin/bash
set -e

echo "📦 Устанавливаю официальные пакеты..."
sudo pacman -S --needed - < pkglist.txt

echo "📦 Устанавливаю пакеты из AUR (через yay)..."
if [ -s aur-pkglist.txt ]; then
    if command -v yay &> /dev/null; then
        yay -S --needed - < aur-pkglist.txt
    else
        echo "⚠️ yay не найден. Установи его и запусти скрипт снова."
        exit 1
    fi
fi

echo "🦀 Собираю и устанавливаю True Rice GUI..."
if [ -d "rice-gui" ]; then
    cd rice-gui
    cargo build --release
    mkdir -p ~/.local/bin
    cp target/release/rice-gui ~/.local/bin/
    cd ..
    echo "✅ Rice GUI установлен в ~/.local/bin/"
else
    echo "⚠️ Папка rice-gui не найдена, пропускаю."
fi

echo "✅ Все пакеты установлены!"

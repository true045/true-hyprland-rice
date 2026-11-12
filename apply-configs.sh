#!/bin/bash
set -e

if ! command -v stow &> /dev/null; then
    echo "❌ GNU Stow не установлен. Установи его: sudo pacman -S stow"
    exit 1
fi

echo "🔗 Создаю символические ссылки через Stow..."
cd config
for dir in */; do
    package=$(basename "$dir")
    echo "-> Применяю $package..."
    stow -v -t "$HOME/.config" "$package"
done
cd ..

if [ -f ".zshrc" ]; then
    echo "-> Применяю .zshrc..."
    ln -sf "$PWD/.zshrc" "$HOME/.zshrc"
fi
echo "✅ Все конфиги применены!"

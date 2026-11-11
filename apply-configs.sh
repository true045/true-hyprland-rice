#!/bin/bash
echo "🔗 Создаю символические ссылки через Stow..."

# Применяем конфиги из папки config в ~/.config
cd config
for dir in */; do
    package=$(basename "$dir")
    echo "-> Применяю $package..."
    stow -v -t "$HOME/.config" "$package"
done
cd ..

# Применяем .zshrc (если он есть)
if [ -f ".zshrc" ]; then
    echo "-> Применяю .zshrc..."
    ln -sf "$PWD/.zshrc" "$HOME/.zshrc"
fi
echo "✅ Все конфиги применены!"

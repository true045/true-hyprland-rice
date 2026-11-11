# True Hyprland Rice (Arch Linux)

Мой персональный, легковесный и кастомизированный рабочий стол на базе Hyprland. 
Оптимизирован для слабых ноутбуков (тестировалось на Intel Pentium4401U + 4GB RAM). 

## Особенности (Features)
- **WM**: Hyprland (Lua config)
- **Bar**: Waybar (кастомный дизайн)
- **Terminal**: Kitty
- **Тема**: Темно-бордовая (Dark/Burgundy)
- **Звук**: Hyprpwcenter (кастомный CSS)
- **Обои**: Автоматически подтягиваются из репозитория

##  Скриншоты


## Установка (Installation)

> ⚠️ **Внимание:** Инструкция для чистой установки Arch Linux (без графической оболочки).

### 1. Подготовка
Убедись, что у тебя есть интернет и установлены базовые пакеты:
```bash
sudo pacman -Syu --needed base-devel git stow
### 2. Установка AUR-хелпера (yay)
git clone https://aur.archlinux.org/yay.git
cd yay
makepkg -si
cd ..
rm -rf yay
### 3. Клонирование райса
git clone https://github.com/true045/true-hyprland-rice.git
cd true-hyprland-rice
### 4. Установка пакетов
./install-packages.sh
### 5. Применение конфигов
./apply-configs.sh
### 6. Запуск
sudo systemctl enable --now NetworkManager
Hyprland


  !!! Основные горячие клавиши (Keybinds) !!!

    Super + Q — Открыть терминал (Kitty)

    Super + C — Закрыть активное окно

    Super + D — Меню запуска (Fuzzel)

    Super + 1-9 — Переключение рабочих столов

    Super + E - открытие файлового менеджера (Dolphine)

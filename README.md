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
- - **GUI Менеджер**: Собственная программа на Rust (`rice-gui`) для управления обоями, Waybar, звуком, сетью, яркостью и цветами.

##  Скриншоты
<img width="1366" height="768" alt="изображение" src="https://github.com/user-attachments/assets/bfcd7e22-3ada-4a1a-904c-83c02583d2e3" /> 
<img width="1360" height="756" alt="изображение" src="https://github.com/user-attachments/assets/9ba06010-f303-42c9-af3b-12aa6f8864db" />



## Установка (Installation)

> ⚠️ **Внимание:** Инструкция для чистой установки Arch Linux (без графической оболочки).

### 1. Подготовка
Убедись, что у тебя есть интернет и установлены базовые пакеты (включая компилятор Rust для нашего GUI и Stow для конфигов):
```bash
sudo pacman -Syu --needed base-devel git stow rust

### 2. Установка AUR-хелпера (yay)

git clone https://aur.archlinux.org/yay.git
cd yay
makepkg -si
cd ..
rm -rf yay

### 3. Клонирование райса

git clone https://github.com/true045/true-hyprland-rice.git
cd true-hyprland-rice

### 4. Установка пакетов и сборка GUI

Скрипт автоматически установит все нужные программы (Hyprland, Waybar, Kitty, PipeWire и т.д.), а также соберет и установит графическую программу на Rust:
bash

./install-packages.sh

## 5. Применение конфигов

Скрипт создаст символические ссылки на конфиги в вашей системе через stow:
bash

./apply-configs.sh

### 6. Включение служб и запуск

Включи сеть, звук (для корректной работы Waybar и GUI) и запусти Hyprland:
bash

sudo systemctl enable --now NetworkManager
systemctl --user enable --now pipewire pipewire-pulse wireplumber
Hyprland

 Основные горячие клавиши (Keybinds)
Управление окнами и программами

    Super + Q — Открыть терминал (Kitty)

    Super + R — Открыть True Rice Manager (наше GUI на Rust)

    Super + D — Меню запуска (Fuzzel)

    Super + C — Закрыть активное окно

    Super + 1-9 — Переключение рабочих столов
    Super + E — Файловый менеджер (Dolphin)

#!/bin/bash

# Получаем текущую яркость в процентах (без знака %)
current=$(brightnessctl -m | awk -F, '{print $4}' | tr -d '%')

# Показываем окно zenity со шкалой от 1 до 100 и шагом 10
new=$(zenity --scale \
    --title="Яркость экрана" \
    --text="Используйте стрелки ← → или ↑ ↓ для изменения" \
    --min-value=1 \
    --max-value=100 \
    --value="$current" \
    --step=10)

# Если пользователь нажал "Отмена" или закрыл окно, ничего не делаем
if [ -n "$new" ]; then
    brightnessctl set "${new}%"
fi


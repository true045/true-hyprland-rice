use gtk4::glib;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation, Stack, StackTransitionType, Separator, gdk, CssProvider, FileDialog, ColorDialog, CheckButton, Scale, Adjustment, ScrolledWindow, PolicyType, Entry};
use std::process::Command;
use std::fs;
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;
use regex::Regex;

// --- ФУНКЦИИ ДЛЯ РАБОТЫ С КОНФИГАМИ ---
fn update_hyprland_wallpaper(path: &str) {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.config/hypr/hyprland.lua", home);
    if let Ok(content) = fs::read_to_string(&config_path) {
        let re = Regex::new(r#"swaybg -i [^"]+"#).unwrap();
        let new_cmd = format!("swaybg -i {}", path);
        let new_content = if re.is_match(&content) {
            re.replace_all(&content, new_cmd.as_str()).to_string()
        } else {
            format!("{}\nhl.exec_cmd(\"swaybg -i {}\")\n", content, path)
        };
        let mut file = fs::File::create(config_path).unwrap();
        file.write_all(new_content.as_bytes()).unwrap();
    }
}

fn get_active_waybar_modules() -> Vec<String> {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.config/waybar/config.jsonc", home);
    let mut modules = Vec::new();
    if let Ok(content) = fs::read_to_string(&config_path) {
        let re = Regex::new(r#""modules-right"\s*:\s*\[(.*?)\]"#).unwrap();
        if let Some(caps) = re.captures(&content) {
            let list_str = &caps[1];
            for item in list_str.split(',') {
                let clean = item.trim().trim_matches('"').trim();
                if !clean.is_empty() { modules.push(clean.to_string()); }
            }
        }
    }
    modules
}

fn update_waybar_config(height: i32, is_bottom: bool, enabled_modules: Vec<String>, managed_modules: Vec<String>) {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.config/waybar/config.jsonc", home);
    if let Ok(content) = fs::read_to_string(&config_path) {
        let re_height = Regex::new(r#""height":\s*\d+"#).unwrap();
        let new_height = format!("\"height\": {}", height);
        let mut new_content = re_height.replace_all(&content, new_height.as_str()).to_string();

        let pos_str = if is_bottom { "bottom" } else { "top" };
        let re_pos = Regex::new(r#""position":\s*"\w+""#).unwrap();
        let new_pos = format!("\"position\": \"{}\"", pos_str);
        new_content = re_pos.replace_all(&new_content, new_pos.as_str()).to_string();

        let re_modules = Regex::new(r#""modules-right"\s*:\s*\[(.*?)\]"#).unwrap();
        if let Some(caps) = re_modules.captures(&new_content) {
            let current_str = &caps[1];
            let current_list: Vec<String> = current_str.split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .filter(|s| !s.is_empty()).collect();

            let mut final_list = Vec::new();
            for module in current_list {
                if !managed_modules.contains(&module) { final_list.push(format!("\"{}\"", module)); }
            }
            for mod_name in enabled_modules { final_list.push(format!("\"{}\"", mod_name)); }
            let new_array = format!("\"modules-right\": [{}]", final_list.join(", "));
            new_content = re_modules.replace(&new_content, new_array.as_str()).to_string();
        }
        let mut file = fs::File::create(config_path).unwrap();
        file.write_all(new_content.as_bytes()).unwrap();
        let _ = Command::new("sh").args(["-c", "killall waybar; sleep 0.5; waybar &"]).spawn();
    }
}

// --- ФУНКЦИИ ЗВУКА, ЯРКОСТИ И ЦВЕТОВ ---
fn get_current_volume() -> f64 {
    let output = Command::new("sh").args(["-c", "pactl get-sink-volume @DEFAULT_SINK@ | grep -o -P '(?<=/).*(?=%)' | head -1"]).output().expect("Err");
    String::from_utf8_lossy(&output.stdout).trim().parse::<f64>().unwrap_or(50.0)
}

fn get_current_brightness() -> f64 {
    let output = Command::new("sh").args(["-c", "brightnessctl -m | awk -F, '{print $4}' | tr -d '%'"]).output().expect("Err");
    String::from_utf8_lossy(&output.stdout).trim().parse::<f64>().unwrap_or(50.0)
}

fn apply_fuzzel_color(color: gdk::RGBA, transparent: bool) {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.config/fuzzel/fuzzel.ini", home);
    let alpha = if transparent { "cc" } else { "ff" };
    let hex_color = format!("#{:02x}{:02x}{:02x}{}", (color.red() * 255.0) as u8, (color.green() * 255.0) as u8, (color.blue() * 255.0) as u8, alpha);
    
    if let Ok(content) = fs::read_to_string(&config_path) {
        let re_bg = Regex::new(r"(?m)^background\s*=\s*.*$").unwrap();
        let mut new_content = re_bg.replace_all(&content, format!("background={}", hex_color)).to_string();
        let lighter_hex = format!("#{:02x}{:02x}{:02x}{}", ((color.red() * 255.0 + 30.0).min(255.0)) as u8, ((color.green() * 255.0 + 30.0).min(255.0)) as u8, ((color.blue() * 255.0 + 30.0).min(255.0)) as u8, alpha);
        let re_sel = Regex::new(r"(?m)^selection\s*=\s*.*$").unwrap();
        new_content = re_sel.replace_all(&new_content, format!("selection={}", lighter_hex)).to_string();
        let mut file = fs::File::create(config_path).unwrap();
        file.write_all(new_content.as_bytes()).unwrap();
    }
}

fn apply_system_color(color: gdk::RGBA) {
    let home = std::env::var("HOME").unwrap();
    let gtk4_css = format!("{}/.config/gtk-4.0/gtk.css", home);
    let hex_color = format!("#{:02x}{:02x}{:02x}", (color.red() * 255.0) as u8, (color.green() * 255.0) as u8, (color.blue() * 255.0) as u8);
    let css_content = format!(
        "/* Системные цвета, сгенерированные True Rice Manager */\n\
         @define-color accent_bg_color {};\n\
         @define-color accent_color {};\n\
         window.background {{ background-color: {}; }}\n",
        hex_color, hex_color, hex_color
    );
    let mut file = fs::File::create(gtk4_css).unwrap();
    file.write_all(css_content.as_bytes()).unwrap();
}

fn apply_hypr_border_color(is_active: bool, color: gdk::RGBA) {
    let r = (color.red() * 255.0) as u8;
    let g = (color.green() * 255.0) as u8;
    let b = (color.blue() * 255.0) as u8;
    let a = (color.alpha() * 255.0) as u8;
    let hex = format!("rgba({:02x}{:02x}{:02x}{:02x})", r, g, b, a);

    let keyword = if is_active { "general:col.active_border" } else { "general:col.inactive_border" };
    let _ = Command::new("hyprctl").args(["keyword", keyword, &hex]).output();

    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.config/hypr/hyprland.lua", home);
    if let Ok(content) = fs::read_to_string(&config_path) {
        let pattern = if is_active { r"(?m)^\s*active_border\s*=\s*.*$" } else { r"(?m)^\s*inactive_border\s*=\s*.*$" };
        let re = Regex::new(pattern).unwrap();
        let new_line = format!("    {} = \"{}\"", if is_active {"active_border"} else {"inactive_border"}, hex);

        let new_content = if re.is_match(&content) {
            re.replace_all(&content, new_line.as_str()).to_string()
        } else {
            format!("{}\nhl.config({{ general = {{ col = {{ {} = \"{}\" }} }} }})\n", content, if is_active {"active_border"} else {"inactive_border"}, hex)
        };
        let mut file = fs::File::create(config_path).unwrap();
        file.write_all(new_content.as_bytes()).unwrap();
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id("org.true045.ricegui").build();

    app.connect_activate(|app| {
        let provider = CssProvider::new();
        provider.load_from_data(
            "
            button { background-color: #4A0E1D; background-image: none; color: #F0D0D8; border-radius: 8px; border: 1px solid #7A1A32; font-weight: bold; padding: 8px; box-shadow: none; }
            button:hover { background-color: #5E1327; }
            button:active { background-color: #B3003C; }
            checkbutton { color: #f0d0d8; margin-top: 5px; }
            scale trough { background-color: rgba(80, 0, 35, 0.5); }
            scale highlight { background-color: #b3003c; }
            entry { background-color: rgba(0, 0, 0, 0.3); color: #f0d0d8; border: 1px solid rgba(179, 0, 60, 0.5); }
            "
        );
        gtk::style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let window = ApplicationWindow::builder().application(app).title("True Rice Manager").default_width(800).default_height(550).build();
        let main_box = GtkBox::new(Orientation::Horizontal, 0);
        
        let sidebar = GtkBox::new(Orientation::Vertical, 5);
        sidebar.set_margin_top(15); sidebar.set_margin_bottom(15); sidebar.set_margin_start(15); sidebar.set_margin_end(10); sidebar.set_width_request(180);
        
        // ИСПРАВЛЕННЫЙ САЙДБАР: добавлен vexpand(true)
        let sidebar_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .vexpand(true) // <--- ВОТ ЭТО ВЕРНЕТ СКРОЛЛ
            .child(&sidebar)
            .build();
        sidebar_scroll.set_width_request(200);

        let stack = Stack::new();
        stack.set_transition_type(StackTransitionType::Crossfade);
        stack.set_hexpand(true); stack.set_vexpand(true); stack.set_margin_top(15); stack.set_margin_bottom(15); stack.set_margin_start(15); stack.set_margin_end(15);

        // ВКЛАДКА 1: ОБОИ
        let page_wall = GtkBox::new(Orientation::Vertical, 10);
        let lbl_wall = Label::new(Some("Управление обоями")); lbl_wall.add_css_class("title-1");
        let btn_select_wall = Button::with_label("Выбрать обои");
        let chk_autostart = CheckButton::with_label("Сохранить в автозапуск (Hyprland)"); chk_autostart.set_active(true);
        let window_clone = window.clone(); let chk_clone = chk_autostart.clone();
        btn_select_wall.connect_clicked(move |_| {
            let dialog = FileDialog::builder().title("Выберите обои").build();
            let window_inner = window_clone.clone(); let chk_inner = chk_clone.clone();
            dialog.open(Some(&window_inner), gtk::gio::Cancellable::NONE, move |result| {
                if let Ok(file) = result { if let Some(path) = file.path() {
                    let path_str = path.to_string_lossy().to_string();
                    let _ = Command::new("killall").arg("swaybg").output();
                    let _ = Command::new("swaybg").args(["-i", &path_str]).spawn();
                    if chk_inner.is_active() { update_hyprland_wallpaper(&path_str); }
                }}
            });
        });
        page_wall.append(&lbl_wall); page_wall.append(&btn_select_wall); page_wall.append(&chk_autostart);

        // ВКЛАДКА 2: WAYBAR
        let page_waybar = GtkBox::new(Orientation::Vertical, 10);
        let lbl_waybar = Label::new(Some("Настройка Waybar")); lbl_waybar.add_css_class("title-1");
        let lbl_height = Label::new(Some("Высота панели (px):")); lbl_height.set_halign(gtk::Align::Start);
        let adj = Adjustment::new(30.0, 20.0, 60.0, 1.0, 5.0, 0.0);
        let scale_height = Scale::new(Orientation::Horizontal, Some(&adj)); scale_height.set_draw_value(true);
        let chk_bottom = CheckButton::with_label("Разместить внизу экрана");
        let active_mods = get_active_waybar_modules();
        let standard_modules = ["pulseaudio", "network", "backlight", "battery"];
        let chk_vol = CheckButton::with_label("Громкость (pulseaudio)"); chk_vol.set_active(active_mods.contains(&"pulseaudio".to_string()));
        let chk_net = CheckButton::with_label("Сеть (network)"); chk_net.set_active(active_mods.contains(&"network".to_string()));
        let chk_bright = CheckButton::with_label("Яркость (backlight)"); chk_bright.set_active(active_mods.contains(&"backlight".to_string()));
        let chk_batt = CheckButton::with_label("Батарея (battery)"); chk_batt.set_active(active_mods.contains(&"battery".to_string()));
        let box_custom = GtkBox::new(Orientation::Vertical, 5);
        let custom_checkboxes: Rc<RefCell<Vec<CheckButton>>> = Rc::new(RefCell::new(Vec::new()));
        for mod_name in &active_mods {
            if !standard_modules.contains(&mod_name.as_str()) {
                let chk = CheckButton::with_label(mod_name); chk.set_active(true);
                box_custom.append(&chk); custom_checkboxes.borrow_mut().push(chk);
            }
        }
        let entry_custom = Entry::new(); entry_custom.set_placeholder_text(Some("напр: clock, tray"));
        let btn_add_custom = Button::with_label("Добавить утилиту");
        let cc_clone = custom_checkboxes.clone(); let box_custom_clone = box_custom.clone(); let entry_clone = entry_custom.clone();
        btn_add_custom.connect_clicked(move |_| {
            let text = entry_clone.text().to_string();
            if !text.is_empty() && !cc_clone.borrow().iter().any(|c| c.label().unwrap() == text) {
                let chk = CheckButton::with_label(&text); chk.set_active(true);
                box_custom_clone.append(&chk); cc_clone.borrow_mut().push(chk); entry_clone.set_text("");
            }
        });
        let btn_apply_waybar = Button::with_label("Применить и перезапустить");
        let scale_clone = scale_height.clone(); let chk_bottom_clone = chk_bottom.clone();
        let chk_vol_clone = chk_vol.clone(); let chk_net_clone = chk_net.clone();
        let chk_bright_clone = chk_bright.clone(); let chk_batt_clone = chk_batt.clone();
        let cc_apply = custom_checkboxes.clone();
        btn_apply_waybar.connect_clicked(move |_| {
            let height = scale_clone.value() as i32; let is_bottom = chk_bottom_clone.is_active();
            let mut enabled = Vec::new();
            let mut managed = vec!["pulseaudio".to_string(), "network".to_string(), "backlight".to_string(), "battery".to_string()];
            if chk_vol_clone.is_active() { enabled.push("pulseaudio".to_string()); }
            if chk_net_clone.is_active() { enabled.push("network".to_string()); }
            if chk_bright_clone.is_active() { enabled.push("backlight".to_string()); }
            if chk_batt_clone.is_active() { enabled.push("battery".to_string()); }
            for chk in cc_apply.borrow().iter() {
                let label = chk.label().unwrap().to_string(); managed.push(label.clone());
                if chk.is_active() { enabled.push(label); }
            }
            update_waybar_config(height, is_bottom, enabled, managed);
        });
        page_waybar.append(&lbl_waybar); page_waybar.append(&lbl_height); page_waybar.append(&scale_height); page_waybar.append(&chk_bottom);
        page_waybar.append(&Separator::new(Orientation::Horizontal)); page_waybar.append(&Label::new(Some("Стандартные утилиты:")));
        page_waybar.append(&chk_vol); page_waybar.append(&chk_net); page_waybar.append(&chk_bright); page_waybar.append(&chk_batt);
        page_waybar.append(&Separator::new(Orientation::Horizontal)); page_waybar.append(&Label::new(Some("Кастомные утилиты:")));
        page_waybar.append(&box_custom); page_waybar.append(&entry_custom); page_waybar.append(&btn_add_custom);
        page_waybar.append(&Separator::new(Orientation::Horizontal)); page_waybar.append(&btn_apply_waybar);

        // СКРОЛЛ ДЛЯ WAYBAR
        let waybar_scroll = ScrolledWindow::builder().hscrollbar_policy(PolicyType::Never).vscrollbar_policy(PolicyType::Automatic).vexpand(true).hexpand(true).child(&page_waybar).build();

        // ВКЛАДКА 3: ЗВУК
        let page_sound = GtkBox::new(Orientation::Vertical, 15);
        let lbl_sound = Label::new(Some("Управление звуком")); lbl_sound.add_css_class("title-1");
        let lbl_vol = Label::new(Some("Громкость:")); lbl_vol.set_halign(gtk::Align::Start);
        let current_vol = get_current_volume();
        let adj_vol = Adjustment::new(current_vol, 0.0, 100.0, 1.0, 5.0, 0.0);
        let scale_vol = Scale::new(Orientation::Horizontal, Some(&adj_vol)); scale_vol.set_draw_value(true);
        scale_vol.connect_value_changed(|scale| { let vol = scale.value() as i32; let _ = Command::new("pactl").args(["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", vol)]).output(); });
        let box_sound_btns = GtkBox::new(Orientation::Horizontal, 10);
        let btn_mute = Button::with_label("Mute / Unmute");
        btn_mute.connect_clicked(|_| { let _ = Command::new("pactl").args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"]).output(); });
        let btn_open_pwcenter = Button::with_label("Открыть Hyprpwcenter");
        btn_open_pwcenter.connect_clicked(|_| { let _ = Command::new("hyprpwcenter").spawn(); });
        box_sound_btns.append(&btn_mute); box_sound_btns.append(&btn_open_pwcenter);
        page_sound.append(&lbl_sound); page_sound.append(&lbl_vol); page_sound.append(&scale_vol);
        page_sound.append(&Separator::new(Orientation::Horizontal)); page_sound.append(&box_sound_btns);

        // ВКЛАДКА 4: СЕТЬ
        let page_network = GtkBox::new(Orientation::Vertical, 15);
        let lbl_network = Label::new(Some("Управление сетью")); lbl_network.add_css_class("title-1");
        let btn_nmtui = Button::with_label("Открыть nmtui (в терминале)");
        btn_nmtui.connect_clicked(|_| { let _ = Command::new("kitty").args(["-e", "nmtui"]).spawn(); });
        let btn_nm_gui = Button::with_label("Открыть Графический редактор");
        btn_nm_gui.connect_clicked(|_| { let _ = Command::new("nm-connection-editor").spawn(); });
        page_network.append(&lbl_network); page_network.append(&btn_nmtui); page_network.append(&btn_nm_gui);

        // ВКЛАДКА 5: ЯРКОСТЬ
        let page_brightness = GtkBox::new(Orientation::Vertical, 15);
        let lbl_brightness = Label::new(Some("Яркость экрана")); lbl_brightness.add_css_class("title-1");
        let lbl_bright = Label::new(Some("Уровень подсветки:")); lbl_bright.set_halign(gtk::Align::Start);
        let current_bright = get_current_brightness();
        let adj_bright = Adjustment::new(current_bright, 1.0, 100.0, 1.0, 10.0, 0.0);
        let scale_bright = Scale::new(Orientation::Horizontal, Some(&adj_bright)); scale_bright.set_draw_value(true);
        scale_bright.connect_value_changed(|scale| { let bright = scale.value() as i32; let _ = Command::new("brightnessctl").args(["set", &format!("{}%", bright)]).output(); });
        page_brightness.append(&lbl_brightness); page_brightness.append(&lbl_bright); page_brightness.append(&scale_bright);

        // ВКЛАДКА 6: FUZZEL
        let page_fuzzel = GtkBox::new(Orientation::Vertical, 15);
        let lbl_fuzzel = Label::new(Some("Настройки Fuzzel")); lbl_fuzzel.add_css_class("title-1");
        let selected_color = Rc::new(RefCell::new(gdk::RGBA::new(0.29, 0.05, 0.11, 1.0)));
        let chk_fuzzel_transparent = CheckButton::with_label("Добавить прозрачность (альфа-канал)");
        let btn_fuzzel_color = Button::with_label("Сменить цвет темы Fuzzel");
        let window_f_clone = window.clone(); let color_f_clone = selected_color.clone(); let chk_transp_clone = chk_fuzzel_transparent.clone();
        btn_fuzzel_color.connect_clicked(move |_| {
            let dialog = ColorDialog::builder().title("Выберите цвет для Fuzzel").build();
            let win = window_f_clone.clone(); let c_clone = color_f_clone.clone(); let t_clone = chk_transp_clone.clone();
            dialog.choose_rgba(Some(&win), None, gtk::gio::Cancellable::NONE, move |result| {
                if let Ok(color) = result { *c_clone.borrow_mut() = color; apply_fuzzel_color(color, t_clone.is_active()); }
            });
        });
        let color_transp_clone = selected_color.clone();
        chk_fuzzel_transparent.connect_toggled(move |chk| { apply_fuzzel_color(color_transp_clone.borrow().clone(), chk.is_active()); });
        let btn_edit_fuzzel = Button::with_label("Открыть fuzzel.ini");
        btn_edit_fuzzel.connect_clicked(|_| {
            let home = std::env::var("HOME").unwrap();
            let _ = Command::new("kitty").args(["-e", "nano", &format!("{}/.config/fuzzel/fuzzel.ini", home)]).spawn();
        });
        page_fuzzel.append(&lbl_fuzzel); page_fuzzel.append(&btn_fuzzel_color); page_fuzzel.append(&chk_fuzzel_transparent);
        page_fuzzel.append(&Separator::new(Orientation::Horizontal)); page_fuzzel.append(&btn_edit_fuzzel);

        // ВКЛАДКА 7: ЦВЕТА СИСТЕМЫ
        let page_syscolors = GtkBox::new(Orientation::Vertical, 15);
        let lbl_syscolors = Label::new(Some("Цвета системы")); lbl_syscolors.add_css_class("title-1");
        let btn_sys_color = Button::with_label("Сменить системный цвет (GTK)");
        let window_sys_clone = window.clone();
        btn_sys_color.connect_clicked(move |_| {
            let dialog = ColorDialog::builder().title("Выберите системный цвет").build();
            let win = window_sys_clone.clone();
            dialog.choose_rgba(Some(&win), None, gtk::gio::Cancellable::NONE, move |result| {
                if let Ok(color) = result { apply_system_color(color); }
            });
        });

        let lbl_borders = Label::new(Some("Рамки окон Hyprland")); lbl_borders.add_css_class("title-2");
        let btn_active_border = Button::with_label("Сменить цвет активного окна");
        let window_ac_clone = window.clone();
        btn_active_border.connect_clicked(move |_| {
            let dialog = ColorDialog::builder().title("Цвет активного окна").build();
            let win = window_ac_clone.clone();
            dialog.choose_rgba(Some(&win), None, gtk::gio::Cancellable::NONE, move |result| {
                if let Ok(color) = result { apply_hypr_border_color(true, color); }
            });
        });

        let chk_inactive_border = CheckButton::with_label("Подсветка неактивных окон");
        let btn_inactive_border = Button::with_label("Сменить цвет неактивного окна");
        btn_inactive_border.set_sensitive(false);

        let window_in_clone = window.clone();
        let btn_inactive_clone = btn_inactive_border.clone();
        
        chk_inactive_border.connect_toggled(move |chk| {
            btn_inactive_clone.set_sensitive(chk.is_active());
            if !chk.is_active() {
                apply_hypr_border_color(false, gdk::RGBA::new(0.0, 0.0, 0.0, 0.0));
            } else {
                apply_hypr_border_color(false, gdk::RGBA::new(0.4, 0.4, 0.4, 1.0));
            }
        });

        btn_inactive_border.connect_clicked(move |_| {
            let dialog = ColorDialog::builder().title("Цвет неактивного окна").build();
            let win = window_in_clone.clone();
            dialog.choose_rgba(Some(&win), None, gtk::gio::Cancellable::NONE, move |result| {
                if let Ok(color) = result { apply_hypr_border_color(false, color); }
            });
        });

        page_syscolors.append(&lbl_syscolors);
        page_syscolors.append(&btn_sys_color);
        page_syscolors.append(&Separator::new(Orientation::Horizontal));
        page_syscolors.append(&lbl_borders);
        page_syscolors.append(&btn_active_border);
        page_syscolors.append(&Separator::new(Orientation::Horizontal));
        page_syscolors.append(&chk_inactive_border);
        page_syscolors.append(&btn_inactive_border);

        // СКРОЛЛ ДЛЯ ЦВЕТОВ СИСТЕМЫ (Добавлен vexpand, чтобы работал)
        let syscolors_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .vexpand(true) // Растягиваем на всю высоту
            .hexpand(true)
            .child(&page_syscolors)
            .build();

        // СБОРКА
        stack.add_titled(&page_wall, Some("wallpapers"), "Обои");
        stack.add_titled(&waybar_scroll, Some("waybar"), "Waybar");
        stack.add_titled(&page_sound, Some("sound"), "Звук");
        stack.add_titled(&page_network, Some("network"), "Сеть");
        stack.add_titled(&page_brightness, Some("brightness"), "Яркость");
        stack.add_titled(&page_fuzzel, Some("fuzzel"), "fuzzel");
        // Вставляем скролл вместо самой страницы
        stack.add_titled(&syscolors_scroll, Some("syscolors"), "Цвета системы");

        let pages = [
            ("wallpapers", "Обои"), ("waybar", "Waybar"), ("sound", "Звук"),
            ("network", "Сеть"), ("brightness", "Яркость"), ("fuzzel", "fuzzel"),
            ("syscolors", "Цвета системы"),
        ];

        for (id, btn_label) in pages {
            let btn = Button::with_label(btn_label); btn.set_halign(gtk::Align::Fill);
            let stack_clone = stack.clone(); let id_clone = id.to_string();
            btn.connect_clicked(move |_| { stack_clone.set_visible_child_name(&id_clone); });
            sidebar.append(&btn);
        }

        let separator = Separator::new(Orientation::Vertical);
        main_box.append(&sidebar_scroll); 
        main_box.append(&separator); 
        main_box.append(&stack);

        window.set_child(Some(&main_box)); window.present();
    });
    app.run()
}

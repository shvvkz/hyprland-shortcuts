# Hyprland Shortcuts

Hyprland Shortcuts is a simple and modular CLI tool written in Rust, designed to manage and display keybindings (shortcuts) defined in the `hyprland.conf` configuration file. The goal of this tool is to provide an easy way to visualize, manage, and eventually edit Hyprland shortcuts directly from the terminal.

This tool can also help users create a personalized window to display their shortcuts, or in the future, directly modify them from this window for an even smoother workflow experience.

---

## 🚀 Features

* Display all configured shortcuts with their comments.
* Check for new releases and update the binary automatically.
* Provide version information.
* Allow specifying a custom path for the Hyprland configuration file.

---

## 📚 Usage

```bash
hyprland-shortcuts [FLAGS]
```

### Available Flags:

* `-v`, `--version` : Display current version.
* `-u`, `--update`  : Check and update to the latest version.
* `-d`, `--display` : Display all comments from the configured shortcuts.
* `-p`, `--path <FILE>` : Provide a custom path to the Hyprland configuration file.
* `-h`, `--help`    : Show help information.

### Example:

```bash
hyprland-shortcuts --display
hyprland-shortcuts --display --path /path/to/your/hyprland.conf
```

#### 📖 Best Practice for Comments

To get the most out of this tool, it is recommended to add comments next to your `bind` and `bindm` lines in the following format:

```ini
bind = SUPER, T, exec some-command # SUPER+T: Open terminal
bindm = SUPER, mouse:273, movewindow # SUPER + Mouse Button 1: Move window
```

The tool will read these comments and display them clearly when using the `--display` flag. This helps you quickly understand the purpose of each shortcut directly from the terminal.

---

## 📅 Roadmap

* [x] Initial version with display and update features.
* [x] Modular architecture for easy feature addition.
* [x] Flag support using Clap.
* [ ] Add interactive mode for editing shortcuts.
* [ ] Add export/import functionality for shortcut presets.
* [ ] Add search/filter options when displaying comments.
* [ ] Add validation and linting for hyprland.conf.

---

## 📦 Installation

Clone the repository and install the binary:

```bash
git clone https://github.com/shvvkz/hyprland-shortcuts.git
cd hyprland-shortcuts
make install
make clean
rm -rf hyprland-shortcuts
```

To update the tool at any time, simply run:

```bash
hyprland-shortcuts -u
```

---

## 🤝 Contributing

Feel free to open issues and contribute via pull requests. The goal is to make this tool as useful and user-friendly as possible for all Hyprland users.

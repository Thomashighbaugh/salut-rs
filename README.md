````markdown
# salut-rs: A Terminal Greeter in Rust

`salut-rs` is a terminal greeter application written in Rust, inspired by the C++ project [salut](https://github.com/Wervice/salut) by Wervice. This project provides a customizable welcome message and quick access to frequently used applications when you open a new terminal.

## Motivation

This project was created for several reasons:

- **Rust Practice:** As a learning exercise to gain practical experience with the Rust programming language.
- **NixOS Flakes Integration:** To explore and demonstrate the use of Rust within a reproducible NixOS environment using flakes.
- **Performance and Safety:** To leverage Rust's strengths in performance and memory safety to create a fast and reliable terminal greeter.
- **Direct reimplementation:** To more deeply understand how the original `salut` program works

## Features

- **Customizable Banner:** Displays a welcome banner, rendered using the `figlet` command-line utility for stylized text. The banner text and Figlet font are configurable.
- **Shortcuts:** Provides quick access to user-defined applications via single-key shortcuts.
- **Configuration File:** Uses a TOML configuration file located at `~/.config/salut-rs/config.toml`.
- **Automatic Configuration Creation:** If the configuration file doesn't exist, `salut-rs` creates a default one.
- **Terminal aware:** Adjusts to the size of your terminal window, centers the banner and lays out the menu appropriately
- **Cross Platform:** Uses `crossterm` for cross platform portability

## Installation

### Prerequisites

- **Rust:** You need to have the Rust toolchain installed. The recommended way to install Rust is via `rustup`: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- **figlet:** The `figlet` command-line utility must be installed. Install it using your system's package manager. Examples:
  - Debian/Ubuntu: `sudo apt install figlet`
  - Arch Linux: `sudo pacman -S figlet`
  - macOS (Homebrew): `brew install figlet`
  - Fedora: `sudo dnf install figlet`
  - Nix/NixOS: `nix-env -iA nixos.figlet` (or add `figlet` to your `environment.systemPackages`)
- **Cargo**: Rust's package manager and build tool (automatically installed with Rust)

### Building from Source

1.  **Clone the repository:**

    ```bash
    git clone <repository_url>  # Replace with the actual URL
    cd salut-rs
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

3.  **Install (optional):**

    You can manually copy the compiled binary (located at `target/release/salut-rs`) to a directory in your `$PATH`. For example:

    ```bash
    sudo cp target/release/salut-rs /usr/local/bin/
    ```

    Alternatively, you can use `cargo install`:

    ```bash
     cargo install --path .
    ```

    This will build and install the `salut-rs` to your cargo bin directory (typically `~/.cargo/bin`)

### NixOS (Flakes)

This project is designed to be easily integrated into a NixOS system using flakes. Here's how you can include it:

1.  **Add to your flake inputs:**

    In your `flake.nix`, add `salut-rs` to the `inputs`:

    ```nix
    {
      inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        salut-rs.url = "git+https://github.com/<your_username>/salut-rs"; #REPLACE with your repository URL
        # ... other inputs ...
      };

      # ... rest of your flake ...
    }
    ```

2.  **Include in your configuration:**

    In your `configuration.nix` (or a relevant module), you can add `salut-rs` to your `environment.systemPackages`:

    ```nix
      environment.systemPackages = with pkgs; [
        # ... other packages ...
        inputs.salut-rs.packages.${pkgs.system}.default
      ];

    ```

    You could also install it to your user profile using home-manager

    ```nix
    home.packages = with pkgs; [
        # ... other packages ...
        inputs.salut-rs.packages.${pkgs.system}.default
    ];
    ```

3.  **Rebuild your system:**
    ```bash
     sudo nixos-rebuild switch --flake .#
    ```
    Or, if using home-manager
    ```bash
    home-manager switch --flake .#
    ```

## Configuration

`salut-rs` is configured via a TOML file located at `~/.config/salut-rs/config.toml`. If this file doesn't exist, it will be created automatically with default settings when you first run `salut-rs`.

Here's an example configuration file:

```toml
banner = "Welcome to My System!"
figlet_font = "slant"  # Optional: Specify the figlet font

[shortcuts]
nv.name = "Neovim"
nv.icon = " "
nv.command = "nvim"
nv.description = "Open Neovim editor"

ft.name = "Fastfetch"
ft.icon = " "
ft.command = "fastfetch"
ft.description = "Display system information"

zs.name = "Zsh"
zs.icon = "$ "
zs.command = "zsh"
zs.description = "Start a Zsh shell"

bt.name = "Btop"
bt.icon = " "
bt.command = "btop"
bt.description = "Resource monitor"

# Add more shortcuts as needed...
```
````

**Configuration Options:**

- **`banner`:** (String) The text to be displayed in the banner.
- **`figlet_font`:** (Optional String) The name of the `figlet` font to use. If omitted, it defaults to "chunky". You can see a list of available fonts by running `showfigfonts` in your terminal.
- **`shortcuts`:** (Table) Defines the shortcuts. Each shortcut is a table with the following fields:
  - **`name`:** (String) The name of the application or command.
  - **`icon`:** (String) A UTF-8 icon to represent the shortcut (e.g., a Nerd Font icon). If you don't have icon support in your terminal, you can leave this out.
  - **`command`:** (String) The shell command to execute when the shortcut is selected.
  - **`description`:** (Optional String) A short description of the shortcut.

## Usage

To use `salut-rs`, add the following line to your shell's configuration file (e.g., `~/.bashrc`, `~/.zshrc`):

```bash
/path/to/salut-rs
```

Replace `/path/to/salut-rs` with the actual path to the `salut-rs` executable. If you installed it to a directory in your `$PATH`, you can simply use:

```bash
salut-rs
```

When you open a new terminal, `salut-rs` will:

1.  Clear the screen.
2.  Display the banner (processed by `figlet` to make the cool banner text).
3.  Display the list of available shortcuts.
4.  Show a prompt: `Enter command: `

Enter the key corresponding to a shortcut (e.g., "nv" for Neovim), and press Enter. The associated command will be executed. Enter "q" and press Enter to quit `salut-rs` without executing a shortcut. Which will drop you in your shell, which if you are using `zsh` and did not change the shortcuts, would have the same effect as entering its shortcut.

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue on the GitHub repository. If you'd like to contribute code, please fork the repository and submit a pull request.

## Credits

- **Original C++ Project:** [salut](https://github.com/Wervice/salut) by Wervice.
- **Figlet:** [http://www.figlet.org/](http://www.figlet.org/)
- **Rust:** [https://www.rust-lang.org/](https://www.rust-lang.org/)
- **Crossterm:** [https://crates.io/crates/crossterm](https://crates.io/crates/crossterm)
- **NixOS:** [https://nixos.org/](https://nixos.org/)

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.

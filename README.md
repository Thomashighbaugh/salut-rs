# salut-rs: Terminal Greeter Implemented in Rust

`salut-rs` is a terminal greeter application written in Rust, inspired by the C++ project [salut](https://github.com/Wervice/salut) by Wervice. This project provides a customizable welcome message and quick access to frequently used applications when you open a new terminal and can configure using a sraight forward `.toml` configuration file.

<img src = "https://github.com/Thomashighbaugh/salut-rs/blob/main/.github/assets/default.png" alt ="default appearance of the command when run" width = "800px">
<img src = "https://github.com/Thomashighbaugh/salut-rs/blob/main/.github/assets/1.png" alt ="a variant appearance of the command when run" width = "800px">

## Motivation

This project was created for several reasons:

- **Rust Practice:** As a learning exercise to gain practical experience with the Rust programming language.
- **NixOS Flakes Integration:** To debug my hopefully reproducible NixOS Rust develop environment's template flake in a non-mission critical situation.
- **Performance:** See for myself the performance advantages offered by Rust over C++.
- **Appreciation for the Original Idea:** I really like the idea of `salut` as a means of having a neovim like terminal greeter that is something slightly fancier than a MOTD and hope this re-implementation will be seen by the original author as my homage to that idea that they deserve ultimate credit for.

## Features

- **Customizable Banner:** Displays a welcome banner, rendered using the `figlet` command-line utility for stylized text (so you should have it installed). The banner text and specific Figlet font are configurable.
- **Shortcuts:** Provides quick access to user-defined applications via user defined shortcuts.
- **Configuration File:** Uses a TOML configuration file located at `~/.config/salut-rs/config.toml`.
- **Automatic Configuration Creation:** If the configuration file doesn't exist, `salut-rs` creates a default one for you, which is nicer than erroring out the first time you try to run an application IMHO.
- **Terminal aware:** Adjusts to the size of your terminal window, centers the banner and lays out the menu appropriately, hopefully (if something isn't working drop me an issue or a PR if you fix yourself)
- **Cross Platform:** Uses `crossterm` for terminal emulator support, should work on macOS, Linux and WSL but I use Linux and am not going through an evening of waiting to create a macOS VM to test it or doing the same with a Windows VM (that would be faster) for a project I wrote an afternoon... I still have to make dinner, but it should work just fine in any crossterm supported environment which is `pretty groovy`.

## Installation

### Prerequisites

- **Rust:** You need to have the Rust toolchain installed. The recommended way to install Rust is via `rustup`: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) but if you use specific systems (like NicOS) things may be more or less complicated depending on your setup.
- **figlet:** The `figlet` command-line utility must be installed. Install it using your system's package manager. Examples:
  - Debian/Ubuntu: `sudo apt install figlet`
  - Arch Linux: `sudo pacman -S figlet`
  - macOS (Homebrew): `brew install figlet`
  - Fedora: `sudo dnf install figlet`
  - Nix/NixOS: `nix-env -iA nixos.figlet` (this is bad and you should never do it but instead use `nix profile` or be declarative and add `figlet` to your `environment.systemPackages`)
- **Cargo**: Rust's `radical` package manager and build tool (automatically installed with Rust)

### Building from Source

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/Thomashighbaugh/salut-rs
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

    You could also install it to your user profile using home-manager and declaratively configure it like so:

    ```nix
    home.file.".config/salut-rs/config.toml".text = ''
    banner = "Welcome to My System!"
    figlet_font = "slant"
    banner_color = "Red"
    shortcuts_color = "Yellow"
    prompt_color = "DarkGrey"

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
    bt.icon = " "
    bt.command = "btop"
    bt.description = "Run Btop"
    ```

'';
home.packages = with pkgs; [

# ... other packages ...

inputs.salut-rs.packages.${pkgs.system}.default
];

````

3.  **Rebuild your system:**
    ```bash
     sudo nixos-rebuild switch --flake .#
    ```
    Or, if using home-manager by itself
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
- **`banner_position`:** (Optional Integer) Controls the vertical positioning of the banner. The screen height is divided by this value to determine the starting row for the banner. For example, a value of `2` would position the banner near the middle of the screen, `3` would position it one-third of the way down, and `4` would position it one-quarter of the way down. If omitted, the default value is `4`.
- **`figlet_font`:** (Optional String) The name of the `figlet` font to use. If omitted, it defaults to "chunky". You can see a list of available fonts by running `showfigfonts` in your terminal.
- **`shortcuts`:** (Table) Defines the shortcuts. Each shortcut is a table with the following fields:
  - **`name`:** (String) The name of the application or command.
  - **`icon`:** (String) A UTF-8 icon to represent the shortcut (e.g., a Nerd Font icon). If you don't have icon support in your terminal, you can leave this out.
  - **`command`:** (String) The shell command to execute when the shortcut is selected.
  - **`description`:** (Optional String) A short description of the shortcut.

### Colors Available

As we are using crossterm's abstraction to provide the color to the colored components of the application's interface, users may find having the exact colors that are accepted by their configuration listed for them, so here is a table doing just that:

| Color Name (Case-Insensitive) | `crossterm::style::Color` Variant | Example                                                       | Notes                        |
| :---------------------------- | :-------------------------------- | :------------------------------------------------------------ | :--------------------------- |
| `black`                       | `Color::Black`                    | ![Black](https://placehold.co/15x15/000000/000000.png)        |                              |
| `red`                         | `Color::Red`                      | ![Red](https://placehold.co/15x15/FF0000/FF0000.png)          |                              |
| `green`                       | `Color::Green`                    | ![Green](https://placehold.co/15x15/00FF00/00FF00.png)        |                              |
| `yellow`                      | `Color::Yellow`                   | ![Yellow](https://placehold.co/15x15/FFFF00/FFFF00.png)       |                              |
| `blue`                        | `Color::Blue`                     | ![Blue](https://placehold.co/15x15/0000FF/0000FF.png)         |                              |
| `magenta`                     | `Color::Magenta`                  | ![Magenta](https://placehold.co/15x15/FF00FF/FF00FF.png)      |                              |
| `cyan`                        | `Color::Cyan`                     | ![Cyan](https://placehold.co/15x15/00FFFF/00FFFF.png)         |                              |
| `white`                       | `Color::White`                    | ![White](https://placehold.co/15x15/FFFFFF/FFFFFF.png)        |                              |
| `darkgrey` / `darkgray`       | `Color::DarkGrey`                 | ![Dark Grey](https://placehold.co/15x15/808080/808080.png)    | Both spellings are accepted. |
| `darkred`                     | `Color::DarkRed`                  | ![Dark Red](https://placehold.co/15x15/8B0000/8B0000.png)     |                              |
| `darkgreen`                   | `Color::DarkGreen`                | ![Dark Green](https://placehold.co/15x15/006400/006400.png)   |                              |
| `darkyellow`                  | `Color::DarkYellow`               | ![Dark Yellow](https://placehold.co/15x15/808000/808000.png)  |                              |
| `darkblue`                    | `Color::DarkBlue`                 | ![Dark Blue](https://placehold.co/15x15/00008B/00008B.png)    |                              |
| `darkmagenta`                 | `Color::DarkMagenta`              | ![Dark Magenta](https://placehold.co/15x15/8B008B/8B008B.png) |                              |
| `darkcyan`                    | `Color::DarkCyan`                 | ![Dark Cyan](https://placehold.co/15x15/008B8B/008B8B.png)    |                              |
| `darkwhite`                   | `Color::DarkGrey`                 | ![Dark White](https://placehold.co/15x15/A9A9A9/A9A9A9)       | Displays as Dark Grey        |

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

This project is derived ultimately from the brilliant ideas and hard work of these projects:

- **Original C++ Project:** [salut](https://github.com/Wervice/salut) by Wervice.
- **Figlet:** [http://www.figlet.org/](http://www.figlet.org/)
- **Rust:** [https://www.rust-lang.org/](https://www.rust-lang.org/)
- **Crossterm:** [https://crates.io/crates/crossterm](https://crates.io/crates/crossterm)
- **NixOS:** [https://nixos.org/](https://nixos.org/)

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.

<h1 align="center">GitGet</h1>

<h4 align="center">Find and access repositories quickly with GitGet</h4>

GitGet is a command line tool that allows you to search and display all public repositories of a user on GitHub. With GitGet, you can quickly see what projects a user has created and access them directly from the command line.

To use GitGet, simply run the `gitget` command followed by the GitHub username you want to search for. GitGet will display a list of all the user's public repositories, including the name and description of each repository.

GitGet is a useful tool for anyone who wants to quickly and easily explore a user's projects on GitHub. Try GitGet today and discover all the interesting repositories on GitHub!

# How to use

### Basic Syntax
`gitget <username> [options]`

## Example
![gitget Example](https://raw.githubusercontent.com/SantiagoCalligari/gitget/main/GitGetExample.png)

# How to install

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your computer. GitGet is written in Rust, so you'll need to have Rust installed in order to compile the source code.

## Installation

### Linux

1. Download or clone the GitGet repository to your computer.

2. Open a console or terminal and navigate to the repository directory.

3. Run the command `cargo install --path .` to compile the GitGet source code.

4. This will make a executable gitget in ~/.cargo/bin/gitget, if you want gitget to be executable only in your user put this on your .bashrc or .zshrc

`export PATH=$PATH:~/.cargo/bin/`

5. Reload your configuration file so that the changes take effect. In Bash, you can do this by running the command `source ~/.bashrc`. In Zsh, you can do this by running the command `source ~/.zshrc`.

6. You can now use the `gitget` for your user. 

If you want it to be usable for any user you have to move .cargo/bin/gitget to /usr/bin with 
`sudo mv ~/.cargo/bin/gitget /usr/bin/` 

### Windows


1. Download or clone the GitGet repository to your computer.

2. Open a command prompt and navigate to the repository directory.

3. Run the command `cargo install --path .` to install GitGet. This will compile the source code and create an executable file called `gitget.exe` in the `.cargo\bin` directory.

4. Add the `.cargo\bin` directory to your `PATH` environment variable so that you can execute the executable from anywhere. You can do this by following these steps:

   1. Right-click on the Start button and select "System".
   2. Click on the "Advanced system settings" link on the left.
   3. Click on the "Environment Variables" button.
   4. Under "System Variables", scroll down and find the "Path" variable. Click on it and then click on the "Edit" button.
   5. Click on the "New" button and add the path to the `.cargo\bin` directory. For example: `C:\path\to\gitget\repository\.cargo\bin`.
   6. Click on the "OK" button to save the changes.

5. Close and reopen the command prompt for the changes to the `PATH` environment variable to take effect.

6. You can now use the `gitget` command from anywhere on your computer.

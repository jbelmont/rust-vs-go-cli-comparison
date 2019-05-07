# Writing Rust application and clis in general

A repository exploring differences in building Go and Rust clis.

## Repository Setup Instructions

### Installing Golang

**[GO](https://golang.org/doc/install)**

#### Setup GOPATH
1. Define GOPATH
    1. Using environment variables set `GOPATH=/Users/marcelbelmont/go`
    2. This is an example of MAC OS X
    3. Windows users might have to edit the environment variables through advanced settings
    4. Although the `MSI` installer should do this

#### Go Workspace

A workspace is a directory hierarchy with three directories at its root:

* src contains Go source files
* pkg contains package objects
* bin contains executable commands

* The GOPATH environment variable specifies the location of your workspace.
* It defaults to a directory named go inside your home directory, so $HOME/go on Unix
* %USERPROFILE%\go (usually C:\Users\YourName\go) on Windows.

Create your projects under the workspace

* My path is `/Users/marcelbelmont/go` and I place all my projects under

** `/Users/marcelbelmont/go/src/github.com/jbelmont` **

This is a convention followed in GO to make your project `go get` able

`MAC` users should run the following command:

`mkdir -p ~/go/src/github.com/${github-username}` in a terminal session.

Optionally just open finder and right click and folder structure manually
and Windows users can do the same thing.

### Installing Rustlang

## Installing Rust in Mac OS X / Linux

To install Rust in Linux and Mac OS X run the following command:

```bash
curl https://sh.rustup.rs -sSf | sh
```

For other installation methods please read the following [documentation](https://www.rust-lang.org/en-US/other-installers.html)

Add the cargo binaries to your path by adding the following entry in *~/.profile*, *~/.bash_profile* *~/.bashrc*, *~/.zshrc*:

```bash
echo "export PATH="$HOME/.cargo/bin:$PATH"" >> ~/.zshrc
```

Note here that I appended this using `>>` to *~/.zshrc* but you may choose *~/.bashrc* or another file.

#### Installing Rust in Windows

On Windows, go to [install](https://www.rust-lang.org/install.html) and follow the instructions for installing Rust. 

You will need to install C++ build tools for Visual Studio 2013 or later here but it is best to just install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017)

The tools are in the Other Tools and Frameworks section which looks like this:

![vs build tools](./images/vs-build-tools.png

#### Check Rust Installation

If Rust is installed then you should be able to run the following command in your terminal windows:

```bash
rustc --version
```

#### Updating Rustlang

In order to update the rust binaries you need to run the following command:

```bash
rustup update
```

Now when we run the `--version` options with rustc a new version should be reported if the update found a new version:

```bash
rustc --version
rustc 1.28.0 (9634041f0 2018-07-30)
```

#### Viewing Local Documentation

In order to see local documentation you can run the following command:

```bash
rustup doc
```

#### Cargo Package Manager

Rust comes with a powerful package manager called **cargo**.

You can create new rust packages by using the cargo package manager.

The command you use is: `cargo new` with options and provide a name

#### Create a new binary template package

If you want to create a new rust binary template application then run the following command:

```bash
cargo new PACKAGE_NAME --bin
```

#### Create a new library template package

If you want to create a new rust library template application then run the following command:

```bash
cargo new PACKAGE_NAME --lib
```

## Editor Recommendations and Editor Extensions

I would recommend the `VSCODE Text Editor` it has a nice Go extension

Download [CODE](https://code.visualstudio.com/)

Install Go extension by clicking extension icon and type go in the market place input box then install it

###### Editor Extensions for Golang and Rustlang

* [vscode-rust](https://marketplace.visualstudio.com/items?itemName=kalitaalexey.vscode-rust)
* [ms-vscode.go](https://code.visualstudio.com/docs/languages/go)

* The Go extension will prompt you to install some missing packages you should do this in order to get:

* Lint
* Formatting
* and many more will be done by the EDITOR

## Repo structure information

Please follow along in my official slides for an in-depth explanation of each folder at [Rust vs Golang Talk](https://slides.com/jbelmont80/rust-2#/)

The repository is broken out into a comparison using regular cli tools with no library such as the summation-* folders.

The *ascii-chart* folder explores using Rust Foreign Function Interface (FFI) and in particular linking object files in C that can be called in Rustlang.

The *cli-\** folders explore using libraries such as [clap](https://clap.rs/) for Rust and [Cobra](https://github.com/spf13/cobra) for Golang.

The *web_scraping_with_rust* folder explores different libraries that you can use and techniques to making a web scraping script in Rust.

The *summation_with_tests* folder looks at writing unit tests in Rustlang.

Please look at the slides mentioned earlier for a more in-depth explanation of all the content in this repository.

## Insights from the Golang vs Rustlang comparision

Both Golang and Rustlang are excellent languages to build modern command line applications.

## Recommendations on which language to choose for a cli application

If you are trying to write a Cross Platform CLI Application you will have a much easier time doing so in golang as you can simply do something like this:

```bash
env GOOS=target-OS GOARCH=target-architecture go build package-import-path
```

For a windows build of golang packages with amd64 architecture:

```bash
env GOOS=windows GOARCH=amd64 go build ./...
```

For a linux build of golang packages with arm architecture:

```bash
env GOOS=linux GOARCH=arm go build ./...
```

Read the [Golang docs on go build](https://golang.org/pkg/go/build/) for more information.

Rustlang can be used but it is not as easy to cross compile as Golang.

Rust overall has much better type and more strict type checking than Golang and will catch many issues at the compile step. 

Rust has a great community and I would highly recommend learning Rust to anyone.

*Overall both languages are wonderful.*

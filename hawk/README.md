# hawk
A Solidity static analyzer to identify contract vulnerabilities and gas efficiencies. 


<hr>
<br>

# Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Identified Issues](https://github.com/dio69/capstone/hawk/tree/main/docs)
  - [⚡Optimizations](https://github.com/dio69/hawk/blob/main/docs/identified-optimizations.md)
  - [🪲Vulnerabilities](https://github.com/dio69/hawk/blob/main/docs/identified-vulnerabilities.md)
  - [👍Quality Assurance](https://github.com/dio69/hawk/blob/main/docs/identified-quality-assurance.md)
- [Example Reports]((https://github.com/dio69/capstone/blob/main/hawk/example_report.md))
- [Contributing](#contributing)


&nbsp;
# Installation
First, make sure that you have [Rust installed](https://www.rust-lang.org/tools/install). Then you can choose either of the installation methods by entering the corresponding command in your terminal below.

&nbsp;
### Install from crates.io
```
cargo install hawk
```

&nbsp;
### Install from source
```
git clone https://github.com/dio69/capstone/hawk &&
cd hawk &&
cargo install --path .
```

&nbsp;
# Usage
Now that you have hawk involved, you can use the `hawk` command from anywhere in your terminal. By default, hawk looks for a `./contracts` directory and analyzes every file within the folder. If you would like to specify the directory hawk should use, you can pass the `--path` flag (ex. `hawk --path <path_to_dir>`). 

In the default configuration, hawk runs analysis for every [currently included Optimization, Vulnerability and QA](https://github.com/0xKitsune/hawk#currently-identified-optimizations-vulnerabilities-and-qa), however if you would like to run analysis for select patterns, you can create a `.toml` file for your custom configuration.  Check out the [default hawk.toml configuration](https://github.com/dio69/capstone/hawk/blob/main/hawk.toml) for reference. After creating a custom `.toml` file, make sure to pass the `--toml` flag when running hawk (ex. `hawk --toml <path_to_toml_file>`).

Once hawk runs its analysis, a report will be generated and output as `hawk_report.md`.

At any point you can use `hawk --help` to see a list of all commands and options.

```
Usage: hawk [OPTIONS]

Options:
  -p, --path <PATH>  Path to the directory containing the files hawk will analyze. The default directory is `./contracts`
  -t, --toml <TOML>  Path to the toml file containing the hawk configuration when not using the default settings.
  -h, --help         Print help information
```

&nbsp;



<hr>
<br>

# Contributing
Contributions are welcome and encouraged! If you are interested in contributing, please check out the `Contributing.md` file.

# Basic Plagiarism Detection
## Motivation
Online plagiarism detection tools usually come with a few constraints. Payment could be required, the number of characters to check could be artificially limited, and more. This tool aims to fill a gap where:
- Plagiarism cases are usually simple copy-paste jobs of a few phrases with minor edits,
- Paying for an online tool is unpalatable, and
- Running a command-line tool is simple enough for the user

<!-- TOC depthFrom:2 -->

- [Motivation](#motivation)
- [Project Status (WIP)](#project-status-wip)
- [Installation](#installation)
- [Usage](#usage)
- [Definitions](#definitions)
- [Project Objectives](#project-objectives)
    - [Hard Objectives](#hard-objectives)
    - [Soft (Optimization) Objectives](#soft-optimization-objectives)
- [Technical Details](#technical-details)
    - [Defining Plagiarism](#defining-plagiarism)
    - [Choosing n, s and M](#choosing-n-s-and-m)

<!-- /TOC -->

## Project Status (WIP)
- All options are usable in the executable, and the `equal` metric is quite fast at detecting copy-paste plagiarism of a few words.
- The `lev` metric is too slow for large datasets, but promises more fine-grained control over how different two phrases can be.
- The current output format is very basic and hard to read. It is a project priority.

## Installation
Currently, only building from the source code is supported. 
1. Install the `rust` language toolchain (https://www.rust-lang.org/tools/install).
1. `git clone` this repository to a folder of your choice.
1. Run `cargo build --release` in that folder.
1. The `target/release` folder will contain the `plagiarism-basic` executable to be used.

## Usage
Some setup is required:
1. Two folders need to be created anywhere, a "trusted" folder and an "untrusted" folder.
1. The "trusted" folder may contain any number of files in its top level directory. Each file will be treated as a separate trusted source of text. This is where you might put the text of the top 10 Google search results, for e.g.
1. The "untrusted" folder may contain any number of files in its top level directory. Each file will be treated as a separate untrusted source of text. This is where you would put each separate "submission" from a student, for e.g.
1. The files in both folders must only contain UTF-8 interpretable text. The name of the file will be used in the output of the program, so naming the files appropriately is a good idea. 
1. After these steps are done, the `plagiarism-basic` executable can be run and the path to these folders can be specified in the arguments to the executable.
```
$ ./plagiarism-basic -h
Basic Plagiarism Checker v0.1
Sriram Sami (@frizensami on GitHub)
Checks for plagiarism using very basic metrics between different text files

USAGE:
    plagiarism-basic -m <metric> -n <sensitivity> -s <similarity> -t <trusted-directory> -u <untrusted-directory>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m <metric>                     Sets the metric (function) used for similarity testing. Equal checks that both
                                    strings are equal, and lev uses the Levenshtein distance [possible values: equal,
                                    lev]
    -n <sensitivity>                Sets the number of words required to form a unit of plagiarism checking
    -s <similarity>                 Sets the threshold value for plagiarism to be detected by a chosen metric
    -t <trusted-directory>          Sets the directory containing trusted text files. Each file will be treated as a
                                    separate possible plagiarism source text.
    -u <untrusted-directory>        Sets the directory containing untrusted text files. Each file will be treated as a
                                    separate submission by a separate person.
```

## Definitions
- Untrusted: Something that might be plagiarised
- Trusted: Something that is definitely not plagiarised, but might be a source used in plagiarism

## Project Objectives
### Hard Objectives
- Detect potential cases of plagiarism between multiple untrusted strings (intra-source plagiarism)
- Detect potential cases of plagiarism between trusted source strings and untrusted strings (external-source plagiarism)

### Soft (Optimization) Objectives
1. Minimizing false positive and false negative detection rates
2. Fastest possible detection speed without compromising objective 1.

## Technical Details
### Defining Plagiarism
Informally, two strings that are long enough and with the same number of words that are "similar enough" by a chosen metric are considered to be plagiarised. 

Formally:
- Two separate strings (`s1` and `s1`) consisting of words (a sequence of characters without a space) are considered plagiarised if: 
- Both have `l` words
    - Where `l` < some user-chosen *sensitivity value* `n`
- Where a metric `M` and *similarity value* `s` produces `M(s1, s2) <= s`
- Subject to pre-processing of
    - Removing CR + LF
    - Removing extra spaces (only one space between words)
    - Converting all letters to lowercase
    - Removing all non alphanumeric characters

### Choosing n, s and M
- `n` is a user-chosen value to indicate **how many words** a string needs to be before being considered for plagiarism
- `s` is a user-chosen value to indicate **how similar** the strings have to be before being considered for plagiarism
- `M` is the **metric** used to evaluate the strings for similarity. They can be one of the following
    - `equal`: checks if the strings are equal, ignores `s` value. Uses hashed set intersections, very fast.
    - `lev`: uses the Levenshtein distance between the words, uses the `s` value. Compares between all combinations of string fragments, very slow at the moment.

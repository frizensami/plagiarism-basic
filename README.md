# Basic Plagiarism Detection
## Required Goals
- Detect potential cases of plagiarism between multiple untrusted strings
- Detect potential cases of plagiarism between trusted source strings and untrusted strings

## Optimization Objectives
1. Minimizing false positive and false negative rates
2. Fastest possible detection speed without compromising objective 1.

## Defining Plagiarism
Informally, two strings that are long enough and with the same number of words that are "similar enough" by a chosen metric are considered to be plagiarised. 

Formally:
- Two separate strings (`s1` and `s1`) consisting of words (a sequence of characters without a space) are considered plagiarised if: 
- Both have `l` words (i.e. `|s1| = |s2| = l`)
    - Where `l` < some chosen *sensitivity value* `n`
- Where a metric `M` and *similarity value* `s` produces `M(s1, s2) > s`
- Subject to pre-processing of
    - Removing CR + LF
    - Removing extra spaces (only one space between words)
    - Converting all letters to lowercase
    - Removing all non alphanumeric characters

## Choosing n, s and M
- `n` is a user-chosen value to indicate **how many words** a string needs to be before being considered for plagiarism
- `s` is a user-chosen value to indicate **how similar** the strings have to be before being considered for plagiarism
- `M` is the metric used to evaluate the strings for similarity. They can be one of the following
    - `equal`: checks if the strings are equal, ignores `s` value
    - `hamming`: uses the Hamming distance between the words, uses the `s` value
    - `lev`: uses the Levenshtein distance between the words, uses the `s` value
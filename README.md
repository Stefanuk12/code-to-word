# Code to Word

A helper tool to export all code in a directory to a word document. Designed for A Level Computer Science students to help with their NEA technical solution write up.

## Examples

```
code-to-word.exe -i . --overwrite
```

## Usage

```
Convert your code to word document

Usage: code-to-word.exe [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>
          The input directory to scan for
  -o, --output <OUTPUT>
          The output file to write to [default: output.docx]
      --overwrite
          Whether to override the output file if it already exists
  -e, --extensions <EXTENSIONS>
          Which file extensions to search for and include [default: rs py js ts html css scss md txt]
  -s, --size-font <SIZE_FONT>
          The font size of the code [default: 8]
  -h, --heading-size <HEADING_SIZE>
          The font size of the code headings [default: 12]
  -f, --font-family-heading <FONT_FAMILY_HEADING>
          The font family of the code headings [default: "Calibri Light"]
  -h, --help
          Print help
  -V, --version
          Print version
```
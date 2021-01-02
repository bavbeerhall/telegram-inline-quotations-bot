# telegram-inline-quotations-bot

A template for Telegram inline quotations bot

## Build

```bash
cargo build --release
```

## Usage

```bash
telegram_inline_quotations_bot --file <filename> --token <token>
```

### Options

- -f, --file <filename>    Filename of the quotations

- -t, --token <token>      Bot token

### Example

```bash
~ ./telegram_inline_quotations_bot -f quotations.txt -t 1234567890:ABCDEFGHIJKLMNOPQRSTUVWXYZ01234567U8
```
set dotenv-load := false

# Show all available recipes
# Show all available recipes, also recurses inside nested justfiles.
@_default:
    just --list --unsorted
    printf "\nExamples:\n"
    printf   "=========\n"
    just examples/
    printf "\nDocs:\n"
    printf   "=====\n"
    just docs/

#########
# Setup #
#########

install:
    # Cargo doesn't need an install step.
    just docs/install
    just examples/install

# Download pre-commits and install Git hooks.
pre-commit version="3.3.3":
    curl \
      --output pre-commit.pyz \
      --location \
      "https://github.com/pre-commit/pre-commit/releases/download/v{{ version }}/pre-commit-{{ version }}.pyz"
    python3 pre-commit.pyz install

# Run pre-commit to lint and format files.
lint hook="" *files="":
    python3 pre-commit.pyz run {{ hook }} {{ if files == "" { "--all-files" } else { "--files" } }} {{ files }}

###########
# Recipes #
###########

# Run the program.
run *args:
    cargo run -- {{ args }}

# Run the program with debug logging.
debug *args:
    env RUST_LOG=debug just run {{ args }}

# Run tests.
test *args:
    cargo test {{ args }}

###########
# Release #
###########

# Build the release binary.
release:
    cargo build --release

###########
# Aliases #
###########

alias i := install
alias p := pre-commit
alias l := lint

alias r := run
alias d := debug
alias t := test

alias R := release

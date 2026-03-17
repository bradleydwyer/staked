# pkg-check

Check if a package name is available across 29 registries at once.

## About

When naming a new library or tool, you want to know which registries already have that name. pkg-check queries npm, PyPI, crates.io, and 26 other package registries in parallel, giving you results in about a second.

It also runs as an MCP server, so AI assistants can check name availability directly.

## Installation

**Homebrew (macOS):**
```bash
brew install bradleydwyer/tap/pkg-check
```

**From source (requires Rust 1.85+):**
```bash
cargo install --git https://github.com/bradleydwyer/pkg-check
```

## Usage

Check a name against the 10 most popular registries:

```
$ pkg-check my-cool-lib
my-cool-lib:
  8 available, 2 taken, 0 unknown (998ms)
  available: crates.io, RubyGems, NuGet, Hex, pub.dev, Homebrew, Docker Hub, CocoaPods
  taken: npm, PyPI
```

Check multiple names:

```
$ pkg-check foo bar baz
```

### Options

```
-v, --verbose            Show per-registry detail
-j, --json               JSON output
-a, --all                Check all 29 registries
-r, --registries <IDS>   Comma-separated registry IDs (e.g. npm,pypi,crates)
-l, --languages <LANGS>  Filter registries by language (e.g. rust,python)
    --list-registries    Show all available registries
```

### Verbose output

```
$ pkg-check -v caucus
caucus:
  8 available, 2 taken, 0 unknown (897ms)
  [-] npm                  TAKEN        (73ms)
  [-] PyPI                 TAKEN        (50ms)
  [+] crates.io            AVAILABLE    (50ms)
  [+] RubyGems             AVAILABLE    (770ms)
  ...
```

### JSON output

```
$ pkg-check -j -r npm,crates my-lib
```

Returns structured JSON with per-registry results, browse URLs, and timing.

### MCP server

```
$ pkg-check mcp
```

Exposes three tools over stdio:

- **check_package** - Check a single name
- **check_packages** - Check up to 50 names in bulk
- **list_registries** - List available registries

## Registries

29 registries organized into a default set of 10 popular ones and 19 additional.

**Default (popular):** npm, PyPI, crates.io, RubyGems, NuGet, Hex, pub.dev, Homebrew, Docker Hub, CocoaPods

**Additional:** Homebrew Cask, Hackage, opam, CPAN, LuaRocks, Nimble, DUB, CRAN, Julia General, conda-forge, vcpkg, Snapcraft, deno.land/x, Shards, VPM, CTAN, Pursuit, WordPress Themes, Chocolatey

Registries with scoped/namespaced packages (Maven Central, Go modules, Packagist, Helm, JSR, etc.) are excluded because names aren't globally reserved -- anyone can publish under their own namespace, so "available" isn't meaningful.

Run `pkg-check --list-registries` to see all registries with their IDs, ecosystems, and supported languages.

## License

pkg-check is licensed under the MIT license. See the [`LICENSE`](LICENSE) file for details.

# staked

<p align="center">
  <img src="logos/staked-logo.png" width="256" alt="staked logo" />
</p>

Check if a package name is available across 29 registries at once.

Queries npm, PyPI, crates.io, and 26 other registries in parallel. Results come back in about a second.

## Install

```bash
brew install bradleydwyer/tap/staked
```

Or from source (Rust 1.85+):

```bash
cargo install --git https://github.com/bradleydwyer/staked
```

## Usage

Check a name against the 10 most popular registries:

```
$ staked my-cool-lib
my-cool-lib:
  8 available, 2 taken, 0 unknown (998ms)
  available: crates.io, RubyGems, NuGet, Hex, pub.dev, Homebrew, Docker Hub, CocoaPods
  taken: npm, PyPI
```

Check multiple names:

```
$ staked foo bar baz
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
$ staked -v caucus
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
$ staked -j -r npm,crates my-lib
```

Returns structured JSON with per-registry results, browse URLs, and timing.

## Registries

29 registries: a default set of 10 popular ones, plus 19 additional.

**Default (popular):** npm, PyPI, crates.io, RubyGems, NuGet, Hex, pub.dev, Homebrew, Docker Hub, CocoaPods

**Additional:** Homebrew Cask, Hackage, opam, CPAN, LuaRocks, Nimble, DUB, CRAN, Julia General, conda-forge, vcpkg, Snapcraft, deno.land/x, Shards, VPM, CTAN, Pursuit, WordPress Themes, Chocolatey

Registries with scoped/namespaced packages (Maven Central, Go modules, Packagist, Helm, JSR, etc.) are excluded. Names there aren't globally reserved, so "available" isn't meaningful.

Run `staked --list-registries` to see everything with IDs, ecosystems, and supported languages.

## License

MIT

## More Tools

**Naming & Availability**
- [available](https://github.com/bradleydwyer/available) — AI-powered project name finder (uses parked, staked & published)
- [parked](https://github.com/bradleydwyer/parked) — Domain availability checker (DNS → WHOIS → RDAP)
- [published](https://github.com/bradleydwyer/published) — App store name checker (App Store & Google Play)

**AI Tooling**
- [sloppy](https://github.com/bradleydwyer/sloppy) — AI prose/slop detector
- [caucus](https://github.com/bradleydwyer/caucus) — Multi-LLM consensus engine
- [nanaban](https://github.com/bradleydwyer/nanaban) — Gemini image generation CLI
- [equip](https://github.com/bradleydwyer/equip) — Cross-agent skill manager

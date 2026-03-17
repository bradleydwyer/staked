use crate::types::RegistryInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvailableSignal {
    Http404,
    EmptyResults,
    NullOrError,
    XmlNoEntry,
}

pub struct Registry {
    pub id: &'static str,
    pub name: &'static str,
    pub ecosystem: &'static str,
    pub languages: &'static [&'static str],
    pub check_url_template: &'static str,
    pub browse_url_template: &'static str,
    pub signal: AvailableSignal,
    pub headers: &'static [(&'static str, &'static str)],
    pub popular: bool,
}

pub const REGISTRIES: [Registry; 29] = [
    // --- Popular (10) ---
    Registry {
        id: "npm",
        name: "npm",
        ecosystem: "JavaScript / TypeScript",
        languages: &["javascript", "typescript"],
        check_url_template: "https://registry.npmjs.org/{name}",
        browse_url_template: "https://www.npmjs.com/package/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: true,
    },
    Registry {
        id: "pypi",
        name: "PyPI",
        ecosystem: "Python",
        languages: &["python"],
        check_url_template: "https://pypi.org/pypi/{name}/json",
        browse_url_template: "https://pypi.org/project/{name}/",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: true,
    },
    Registry {
        id: "crates",
        name: "crates.io",
        ecosystem: "Rust",
        languages: &["rust"],
        check_url_template: "https://crates.io/api/v1/crates/{name}",
        browse_url_template: "https://crates.io/crates/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: true,
    },
    Registry {
        id: "rubygems",
        name: "RubyGems",
        ecosystem: "Ruby",
        languages: &["ruby"],
        check_url_template: "https://rubygems.org/api/v1/gems/{name}.json",
        browse_url_template: "https://rubygems.org/gems/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: true,
    },
    Registry {
        id: "nuget",
        name: "NuGet",
        ecosystem: ".NET",
        languages: &["csharp", "fsharp"],
        check_url_template: "https://api.nuget.org/v3-flatcontainer/{name}/index.json",
        browse_url_template: "https://www.nuget.org/packages/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: true,
    },
    Registry {
        id: "hex",
        name: "Hex",
        ecosystem: "Erlang / Elixir",
        languages: &["elixir", "erlang"],
        check_url_template: "https://hex.pm/api/packages/{name}",
        browse_url_template: "https://hex.pm/packages/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: true,
    },
    Registry {
        id: "pub",
        name: "pub.dev",
        ecosystem: "Dart / Flutter",
        languages: &["dart"],
        check_url_template: "https://pub.dev/api/packages/{name}",
        browse_url_template: "https://pub.dev/packages/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: true,
    },
    Registry {
        id: "homebrew",
        name: "Homebrew",
        ecosystem: "macOS / Linux",
        languages: &["multi"],
        check_url_template: "https://formulae.brew.sh/api/formula/{name}.json",
        browse_url_template: "https://formulae.brew.sh/formula/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: true,
    },
    Registry {
        id: "docker_hub",
        name: "Docker Hub",
        ecosystem: "Containers",
        languages: &["multi"],
        check_url_template: "https://hub.docker.com/v2/repositories/library/{name}/",
        browse_url_template: "https://hub.docker.com/_/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: true,
    },
    Registry {
        id: "cocoapods",
        name: "CocoaPods",
        ecosystem: "iOS / macOS",
        languages: &["swift", "objective-c"],
        check_url_template: "https://trunk.cocoapods.org/api/v1/pods/{name}",
        browse_url_template: "https://cocoapods.org/pods/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: true,
    },
    // --- Additional (20) ---
    Registry {
        id: "homebrew_cask",
        name: "Homebrew Cask",
        ecosystem: "macOS (GUI apps)",
        languages: &["multi"],
        check_url_template: "https://formulae.brew.sh/api/cask/{name}.json",
        browse_url_template: "https://formulae.brew.sh/cask/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "hackage",
        name: "Hackage",
        ecosystem: "Haskell",
        languages: &["haskell"],
        check_url_template: "https://hackage.haskell.org/package/{name}/preferred",
        browse_url_template: "https://hackage.haskell.org/package/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "opam",
        name: "opam",
        ecosystem: "OCaml",
        languages: &["ocaml"],
        check_url_template: "https://opam.ocaml.org/packages/{name}/",
        browse_url_template: "https://opam.ocaml.org/packages/{name}/",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "cpan",
        name: "CPAN",
        ecosystem: "Perl",
        languages: &["perl"],
        check_url_template: "https://fastapi.metacpan.org/v1/distribution/{name}",
        browse_url_template: "https://metacpan.org/dist/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "luarocks",
        name: "LuaRocks",
        ecosystem: "Lua",
        languages: &["lua"],
        check_url_template: "https://luarocks.org/api/1/{name}/rockspec.json",
        browse_url_template: "https://luarocks.org/search?q={name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "nimble",
        name: "Nimble",
        ecosystem: "Nim",
        languages: &["nim"],
        check_url_template: "https://nimble.directory/api/packages/{name}",
        browse_url_template: "https://nimble.directory/pkg/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "dub",
        name: "DUB",
        ecosystem: "D",
        languages: &["d"],
        check_url_template: "https://code.dlang.org/api/packages/{name}",
        browse_url_template: "https://code.dlang.org/packages/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "cran",
        name: "CRAN",
        ecosystem: "R",
        languages: &["r"],
        check_url_template: "https://crandb.r-pkg.org/{name}",
        browse_url_template: "https://cran.r-project.org/package={name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "julia",
        name: "Julia General",
        ecosystem: "Julia",
        languages: &["julia"],
        check_url_template: "https://juliahub.com/ui/Packages/General/{name}",
        browse_url_template: "https://juliahub.com/ui/Packages/General/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "conda",
        name: "conda-forge",
        ecosystem: "Python / Data Science",
        languages: &["python", "r"],
        check_url_template: "https://api.anaconda.org/package/conda-forge/{name}",
        browse_url_template: "https://anaconda.org/conda-forge/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "vcpkg",
        name: "vcpkg",
        ecosystem: "C / C++",
        languages: &["c", "cpp"],
        check_url_template: "https://vcpkg.io/en/package/{name}",
        browse_url_template: "https://vcpkg.io/en/package/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "snapcraft",
        name: "Snapcraft",
        ecosystem: "Linux (snap)",
        languages: &["multi"],
        check_url_template: "https://api.snapcraft.io/v2/snaps/info/{name}",
        browse_url_template: "https://snapcraft.io/{name}",
        signal: AvailableSignal::Http404,
        headers: &[("Snap-Device-Series", "16")],
        popular: false,
    },
    Registry {
        id: "deno_land",
        name: "deno.land/x",
        ecosystem: "Deno",
        languages: &["javascript", "typescript"],
        check_url_template: "https://apiland.deno.dev/v2/modules/{name}",
        browse_url_template: "https://deno.land/x/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "crystal",
        name: "Shards",
        ecosystem: "Crystal",
        languages: &["crystal"],
        check_url_template: "https://shardbox.org/shards/{name}",
        browse_url_template: "https://shardbox.org/shards/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "v_lang",
        name: "VPM",
        ecosystem: "V",
        languages: &["v"],
        check_url_template: "https://vpm.vlang.io/packages/{name}",
        browse_url_template: "https://vpm.vlang.io/packages/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "tex_ctan",
        name: "CTAN",
        ecosystem: "TeX / LaTeX",
        languages: &["tex", "latex"],
        check_url_template: "https://ctan.org/json/2.0/pkg/{name}",
        browse_url_template: "https://ctan.org/pkg/{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "purescript",
        name: "Pursuit",
        ecosystem: "PureScript",
        languages: &["purescript"],
        check_url_template: "https://pursuit.purescript.org/packages/purescript-{name}",
        browse_url_template: "https://pursuit.purescript.org/packages/purescript-{name}",
        signal: AvailableSignal::Http404,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "wordpress_themes",
        name: "WordPress Themes",
        ecosystem: "WordPress",
        languages: &["php"],
        check_url_template: "https://api.wordpress.org/themes/info/1.2/?action=theme_information&slug={name}",
        browse_url_template: "https://wordpress.org/themes/{name}/",
        signal: AvailableSignal::NullOrError,
        headers: &[],
        popular: false,
    },
    Registry {
        id: "chocolatey",
        name: "Chocolatey",
        ecosystem: "Windows",
        languages: &["multi"],
        check_url_template: "https://community.chocolatey.org/api/v2/Packages()?%24filter=Id%20eq%20%27{name}%27&%24top=1",
        browse_url_template: "https://community.chocolatey.org/packages/{name}",
        signal: AvailableSignal::XmlNoEntry,
        headers: &[],
        popular: false,
    },
];

pub fn all_registries() -> &'static [Registry] {
    &REGISTRIES
}

pub fn popular_registries() -> Vec<&'static Registry> {
    REGISTRIES.iter().filter(|r| r.popular).collect()
}

pub fn registries_by_ids(ids: &[String]) -> Vec<&'static Registry> {
    REGISTRIES
        .iter()
        .filter(|r| ids.iter().any(|id| id == r.id))
        .collect()
}

pub fn registries_by_languages(langs: &[String]) -> Vec<&'static Registry> {
    REGISTRIES
        .iter()
        .filter(|r| {
            r.languages
                .iter()
                .any(|l| langs.iter().any(|lang| lang == l))
        })
        .collect()
}

pub fn check_url(registry: &Registry, name: &str) -> String {
    let name_for_url = if registry.id == "nuget" {
        name.to_lowercase()
    } else {
        name.to_string()
    };
    registry.check_url_template.replace("{name}", &name_for_url)
}

pub fn browse_url(registry: &Registry, name: &str) -> String {
    registry.browse_url_template.replace("{name}", name)
}

pub fn registry_info(registry: &Registry) -> RegistryInfo {
    RegistryInfo {
        id: registry.id.to_string(),
        name: registry.name.to_string(),
        ecosystem: registry.ecosystem.to_string(),
        languages: registry.languages.iter().map(|l| l.to_string()).collect(),
    }
}

use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    Lua51,
    Lua52,
    Lua53,
    Lua54,
}
pub use self::Version::*;

pub struct Build {
    out_dir: Option<PathBuf>,
    target: Option<String>,
    host: Option<String>,
}

pub struct Artifacts {
    include_dir: PathBuf,
    lib_dir: PathBuf,
    libs: Vec<String>,
}

impl Build {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Build {
        Build {
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("lua-build")),
            target: env::var("TARGET").ok(),
            host: env::var("HOST").ok(),
        }
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, path: P) -> &mut Build {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn target(&mut self, target: &str) -> &mut Build {
        self.target = Some(target.to_string());
        self
    }

    pub fn host(&mut self, host: &str) -> &mut Build {
        self.host = Some(host.to_string());
        self
    }

    pub fn build(&mut self, version: Version) -> Artifacts {
        if version != Lua51 {
            panic!("only Lua 5.1 (Elune) is supported by this crate");
        }

        let target = self.target.as_ref().expect("TARGET not set");
        let host = self.host.as_ref().expect("HOST not set");
        let out_dir = self.out_dir.as_ref().expect("OUT_DIR not set");
        let lib_dir = out_dir.join("lib");
        let include_dir = out_dir.join("include");

        let source_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("elune/liblua");
        let vendor_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor");

        recreate_dir(&lib_dir);
        recreate_dir(&include_dir);

        let generated_dir = out_dir.join("generated");
        recreate_dir(&generated_dir);

        generate_luaconf(&source_dir, &generated_dir, target);

        let mut config = cc::Build::new();
        config
            .target(target)
            .host(host)
            .warnings(false)
            .opt_level(2)
            .cargo_metadata(false)
            .std("c11");

        config.define("LUA_BUILD_EXPORT", None);
        config.define("_GNU_SOURCE", None);

        apply_platform_defines(&mut config, target);

        if cfg!(debug_assertions) {
            config.define("LUA_USE_APICHECK", None);
        }

        config
            .include(&generated_dir)
            .include(source_dir.join("include"))
            .include(&source_dir)
            .include(&vendor_dir);

        config.flag("-w");
        config.flag_if_supported("-fno-common");

        add_source_files(&mut config, &source_dir);

        let lib_name = "lua5.1";
        config.out_dir(&lib_dir).compile(lib_name);

        copy_public_headers(&source_dir, &generated_dir, &include_dir);

        Artifacts {
            lib_dir,
            include_dir,
            libs: vec![lib_name.to_string()],
        }
    }
}

fn recreate_dir(dir: &Path) {
    if dir.exists() {
        fs::remove_dir_all(dir).unwrap();
    }
    fs::create_dir_all(dir).unwrap();
}

fn apply_platform_defines(config: &mut cc::Build, target: &str) {
    if target.contains("linux") || target.ends_with("bsd") {
        config.define("LUA_USE_LINUX", None);
        config.define("LUA_USE_POSIX", None);
    } else if target.contains("apple-darwin") {
        config.define("LUA_USE_LINUX", None);
        config.define("LUA_USE_POSIX", None);
        config.define("LUA_USE_MACOS", None);
    } else if target.contains("windows") {
        config.define("LUA_USE_WINDOWS", None);
    } else {
        config.define("LUA_USE_POSIX", None);
    }
}

fn add_source_files(config: &mut cc::Build, source_dir: &Path) {
    let files = [
        "lapi.c",
        "lauxlib.c",
        "lbaselib.c",
        "lbitlib.c",
        "lcode.c",
        "lcompatlib.c",
        "lcorolib.c",
        "ldblib.c",
        "ldebug.c",
        "ldo.c",
        "ldump.c",
        "lfunc.c",
        "lgc.c",
        "linit.c",
        "liolib.c",
        "llex.c",
        "lmanip.c",
        "lmathlib.c",
        "lmem.c",
        "loadlib.c",
        "lobject.c",
        "lopcodes.c",
        "loslib.c",
        "lparser.c",
        "lreadline.c",
        "lsec.c",
        "lseclib.c",
        "lstate.c",
        "lstatslib.c",
        "lstring.c",
        "lstrlib.c",
        "ltable.c",
        "ltablib.c",
        "ltm.c",
        "lundump.c",
        "lvm.c",
        "lzio.c",
    ];
    for f in &files {
        config.file(source_dir.join(f));
    }
}

fn generate_luaconf(source_dir: &Path, generated_dir: &Path, target: &str) {
    let template_path = source_dir.join("include").join("luaconf.h.in");
    let reader = BufReader::new(fs::File::open(&template_path).unwrap());
    let output_path = generated_dir.join("luaconf.h");
    let mut out = fs::File::create(&output_path).unwrap();

    let is_windows = target.contains("windows");
    let is_linux = target.contains("linux") || target.ends_with("bsd");
    let is_macos = target.contains("apple-darwin");
    let is_posix = is_linux || is_macos || target.contains("apple-ios");

    let dirsep = if is_windows { "\\\\\\\\" } else { "/" };
    let lua_output_name = if is_windows { "lua" } else { "lua5.1" };
    let lua_path = if is_windows {
        ".\\\\?.lua;.\\\\?\\\\init.lua"
    } else {
        "./?.lua;./?/init.lua"
    };
    let lua_cpath = if is_windows {
        ".\\\\?.dll"
    } else {
        "./?.so"
    };

    for line in reader.lines() {
        let line = line.unwrap();
        let processed = process_luaconf_line(
            &line, is_windows, is_linux, is_macos, is_posix, dirsep, lua_output_name, lua_path,
            lua_cpath,
        );
        writeln!(out, "{}", processed).unwrap();
    }
}

fn process_luaconf_line(
    line: &str,
    is_windows: bool,
    is_linux: bool,
    is_macos: bool,
    is_posix: bool,
    dirsep: &str,
    lua_output_name: &str,
    lua_path: &str,
    lua_cpath: &str,
) -> String {
    // First, do all @VAR@ substitutions
    let line = line
        .replace("@PROJECT_VERSION@", "3.2")
        .replace("@LUAI_BITSINT@", "32")
        .replace("@LUA_OUTPUT_NAME@", lua_output_name)
        .replace("@LUA_DIRSEP_ESCAPED@", dirsep)
        .replace("@LUA_PATH_ESCAPED@", lua_path)
        .replace("@LUA_CPATH_ESCAPED@", lua_cpath);

    // Then handle #cmakedefine directives
    if line.starts_with("#cmakedefine ") {
        let rest = line.trim_start_matches("#cmakedefine ");
        let var = rest.split_whitespace().next().unwrap_or("");
        let value_part = rest[var.len()..].trim_start();
        let defined = match var {
            "LUA_USE_WINDOWS" => is_windows,
            "LUA_USE_LINUX" => is_linux,
            "LUA_USE_MACOS" => is_macos,
            "LUA_USE_POSIX" => is_posix,
            "LUAI_BITSINT" => true,
            "LUA_USE_LONGLONG" => false,
            "LUA_USE_CXX_EXCEPTIONS" => false,
            "LUA_USE_CXX_LINKAGE" => false,
            "LUA_USE_SHARED" => false,
            "LUA_USE_READLINE" => false,
            "LUA_DISABLE_LOADLIB" => false,
            _ => false,
        };
        if defined {
            if value_part.is_empty() {
                return format!("#define {}", var);
            } else {
                return format!("#define {} {}", var, value_part);
            }
        } else {
            return format!("/* #undef {} */", var);
        }
    }

    line
}

fn copy_public_headers(source_dir: &Path, generated_dir: &Path, include_dir: &Path) {
    for f in &["lua.h", "lauxlib.h", "lualib.h"] {
        fs::copy(source_dir.join("include").join(f), include_dir.join(f)).unwrap();
    }
    fs::copy(generated_dir.join("luaconf.h"), include_dir.join("luaconf.h")).unwrap();
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn libs(&self) -> &[String] {
        &self.libs
    }

    pub fn print_cargo_metadata(&self) {
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());
        for lib in self.libs.iter() {
            println!("cargo:rustc-link-lib=static={}", lib);
        }
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }
}

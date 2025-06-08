use std::{
    error::Error,
    fs::read_to_string,
    num::ParseIntError,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

// The environmental variables that are usually set by R. These might be needed
// to set manually if we compile extendr-ffi outside of an R session.
// c.f., https://stat.ethz.ch/R-manual/R-devel/library/base/html/EnvVar.html
const ENVVAR_R_HOME: &str = "R_HOME";
const ENVVAR_R_INCLUDE_DIR: &str = "R_INCLUDE_DIR";

struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl FromStr for Version {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.splitn(3, ".").collect::<Vec<_>>();
        Ok(Self {
            major: parts[0].parse::<u8>()?,
            minor: parts[1].parse::<u8>()?,
            patch: parts[2].parse::<u8>()?,
        })
    }
}

fn read_r_ver(path: &Path) -> Result<Version, Box<dyn Error>> {
    let ver_h = path.join("Rversion.h");
    info!("Attempting to read version from {}", ver_h.display());
    let content = read_to_string(ver_h).map_err(|e| {
        error!("Unable to read Rversion.h: {}", e.to_string());
        e
    })?;

    let major = content
        .lines()
        .find(|line| line.starts_with("#define R_MAJOR"))
        .and_then(|line| line.split_whitespace().nth(2))
        .and_then(|v| v.trim_matches('"').to_string().into())
        .unwrap_or_default();

    let minor = content
        .lines()
        .find(|line| line.starts_with("#define R_MINOR"))
        .and_then(|line| line.split_whitespace().nth(2))
        .and_then(|v| v.trim_matches('"').to_string().into())
        .unwrap_or_default();

    let version = Version::from_str(&format!("{major}.{minor}")).map_err(|e| {
        error!("Unable to parse R version: {}", e.to_string());
        e
    })?;

    info!(
        "R version found is: {}.{}.{}",
        version.major, version.minor, version.patch
    );

    Ok(version)
}

impl Version {
    fn get_r_include() -> Result<String, Box<dyn Error>> {
        let r_cmd = match cfg!(windows) {
            true => "R.exe",
            false => "R",
        };

        let mut cmd = Command::new(r_cmd);
        // print the include dir from R
        cmd.args([
            "--vanilla",
            "-s",
            "-e",
            "cat(normalizePath(R.home('include')))",
        ]);

        info!("Running R command: {:?}", cmd);

        let out = cmd.output()?.stdout;

        let res = String::from_utf8(out).map_err(|e| {
            error!("Unable to parse R output");
            e
        })?;

        info!("R command output: {:?}", res);
        Ok(res)
    }

    fn try_new() -> Result<Self, Box<dyn Error>> {
        let include_dir = match std::env::var(ENVVAR_R_INCLUDE_DIR) {
            Ok(v) => {
                let v = if v.is_empty() {
                    warn!("R_INCLUDE_DIR environment variable is empty.");
                    Self::get_r_include()?
                } else {
                    v
                };
                info!("R_INCLUDE_DIR: {v}");
                v
            }
            Err(_) => {
                warn!("R_INCLUDE_DIR not found. Likely being built outside of R.");
                Self::get_r_include()?
            }
        };

        let r_ver = read_r_ver(&Path::new(&include_dir))?;

        Ok(r_ver)
    }
}

struct InstallationPaths {
    r_home: PathBuf,
    version: Version,
}

impl InstallationPaths {
    fn get_r_home() -> Result<String, Box<dyn Error>> {
        let r_cmd = match cfg!(windows) {
            true => "R.exe",
            false => "R",
        };

        let mut cmd = Command::new(r_cmd);
        cmd.args(["-s", "-e", "cat(normalizePath(R.home()))"]);
        info!("Running R command: {cmd:?}");
        let res = String::from_utf8(cmd.output()?.stdout)?;
        info!("R_HOME found at {res}");
        Ok(res)
    }

    fn try_new() -> Result<Self, Box<dyn Error>> {
        // If R_HOME is unset then we try and call R directly.
        let r_home = match std::env::var(ENVVAR_R_HOME) {
            Err(_) => {
                warn!("R_HOME not found. Trying to fetch from R directly.");
                Self::get_r_home()?
            }
            Ok(v) => {
                let v = if v.is_empty() {
                    warn!("R_HOME is empty. Trying to fetch from R directly");
                    Self::get_r_home()?
                } else {
                    v
                };
                info!("R_HOME: {v}");
                v
            }
        };

        Ok(InstallationPaths {
            r_home: Path::new(&r_home).to_path_buf(),
            version: Version::try_new()?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-check-cfg=cfg(r_4_4)");
    println!("cargo:rustc-check-cfg=cfg(r_4_5)");

    // Fetch R_HOME and R version
    let r_paths = match InstallationPaths::try_new() {
        Ok(v) => v,
        Err(_) => {
            warn!("Cannot fetch R version from R. Defaulting to most recent configure flag");
            println!("cargo:rustc-cfg=r_4_5");
            println!("cargo:r_version_major=4");
            println!("cargo:r_version_minor=5");
            println!("cargo:r_version_patch=1");
            return Ok(());
        }
    };

    // Used by extendr-engine becomes DEP_R_R_HOME for clients
    println!("cargo:r_home={}", r_paths.r_home.display());
    println!("cargo:rustc-env=R_HOME={}", r_paths.r_home.display());

    // used by extendr-api
    println!("cargo:r_version_major={}", r_paths.version.major);
    println!("cargo:r_version_minor={}", r_paths.version.minor);
    println!("cargo:r_version_patch={}", r_paths.version.patch);

    let pkg_target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let libdir = match (cfg!(windows), pkg_target_arch.as_str()) {
        // For Windows
        (true, "x86_64") => Path::new(&r_paths.r_home).join("bin").join("x64"),
        (true, "x86") => Path::new(&r_paths.r_home).join("bin").join("i386"),
        (true, _) => {
            return Err("Cannot build extendr-ffi for unknown architecture".into());
        }
        // For Unix-alike
        (false, _) => Path::new(&r_paths.r_home).join("lib"),
    };

    note!("R library paths determined to be at: {libdir:?}");

    if let Ok(r_library) = libdir.canonicalize() {
        println!("cargo:rustc-link-search={}", r_library.display());
    }

    println!("cargo:rustc-link-lib=dylib=R");

    // Set R version specfic config flags
    // use r_4_4 config
    if (r_paths.version.major, r_paths.version.minor) >= (4, 4) {
        println!("cargo:rustc-cfg=r_4_4")
    }

    // use r_4_5 config
    if (r_paths.version.major, r_paths.version.minor) >= (4, 5) {
        println!("cargo:rustc-cfg=r_4_5")
    }

    // Only re-run if the include directory changes
    println!("cargo:rerun-if-env-changed=R_INCLUDE_DIR");

    custom_println!(" Success:", green, "extendr-ffi has been built!");
    Ok(())
}

// Taken from build-print
#[macro_export]
macro_rules! buildprintln {
    () => {
        ::std::println!("cargo:warning=\x1b[2K\r");
    };
    ($($arg:tt)*) => {
        ::std::println!("cargo:warning=\x1b[2K\r{}", ::std::format!($($arg)*))
    }
}

#[macro_export]
macro_rules! custom_println {
    ($prefix:literal, cyan, $($arg:tt)*) => {
        buildprintln!("   \x1b[1m\x1b[36m{}\x1b[0m {}", $prefix, ::std::format!($($arg)+));
    };
    ($prefix:literal, green, $($arg:tt)*) => {
       buildprintln!("   \x1b[1m\x1b[32m{}\x1b[0m {}", $prefix, ::std::format!($($arg)+));
    };
    ($prefix:literal, yellow, $($arg:tt)*) => {
        buildprintln!("   \x1b[1m\x1b[33m{}\x1b[0m {}", $prefix, ::std::format!($($arg)+));
    };
    ($prefix:literal, red, $($arg:tt)*) => {
        buildprintln!("   \x1b[1m\x1b[31m{}\x1b[0m {}", $prefix, ::std::format!($($arg)+));
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        custom_println!("info:", green, $($arg)+);
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        custom_println!("warning:", yellow, $($arg)+);
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        custom_println!("error:", red, $($arg)+);
    }
}

#[macro_export]
macro_rules! note {
    ($($arg:tt)+) => {
        custom_println!("note:", cyan, $($arg)+);
    }
}

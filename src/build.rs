use std::{
    collections::HashMap,
    env,
    ffi::{OsStr, OsString},
    io::ErrorKind,
    path::{Path, PathBuf},
    process::Command,
};

/// Zig build release modes.
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ReleaseMode {
    #[default]
    Auto,
    Fast,
    Safe,
    Small,
}

/// Zig build project optimization modus.
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Optimize {
    #[default]
    Default,
    Debug,
    ReleaseSafe,
    ReleaseFast,
    ReleaseSmall,
}

/// Builder style configuration for a pending Zig build.
pub struct Build {
    path: PathBuf,
    step: Option<OsString>,
    // General options.
    prefix: Option<PathBuf>,
    prefix_lib_dir: Option<PathBuf>,
    prefix_exe_dir: Option<PathBuf>,
    prefix_include_dir: Option<PathBuf>,
    release: Option<ReleaseMode>,
    darling: Option<bool>,
    qemu: Option<bool>,
    glibc_runtimes: Option<PathBuf>,
    rosetta: Option<bool>,
    wasmtime: Option<bool>,
    wine: Option<bool>,
    verbose: bool,
    prominent_compile_errors: bool,
    jobs: Option<usize>,
    maxrss: Option<usize>,
    skip_oom_steps: bool,
    incremental: Option<bool>,
    // Project-specific options.
    target: Option<OsString>,
    cpu: Option<OsString>,
    dynamic_linker: Option<PathBuf>,
    optimize: Option<Optimize>,
    options: Vec<OsString>,
    // Advanced options.
    reference_trace: Option<usize>,
    no_reference_trace: bool,
    build_file: Option<PathBuf>,
    cache_dir: Option<PathBuf>,
    global_cache_dir: Option<PathBuf>,
    zig_lib_dir: Option<PathBuf>,
    build_runner: Option<PathBuf>,
    seed: Option<usize>,
    verbose_link: bool,
    verbose_air: bool,
    verbose_llvm_ir: Option<PathBuf>,
    verbose_llvm_bc: Option<PathBuf>,
    verbose_cimport: bool,
    verbose_cc: bool,
    verbose_llvm_cpu_features: bool,
    // Additional members.
    env_cache: HashMap<String, Option<OsString>>,
}

impl Build {
    /// Creates a new blank set of configurations to build the project specified at the path
    /// `paths`.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: env::current_dir().unwrap().join(path),
            step: None,
            prefix: None,
            prefix_lib_dir: None,
            prefix_exe_dir: None,
            prefix_include_dir: None,
            release: None,
            darling: None,
            qemu: None,
            glibc_runtimes: None,
            rosetta: None,
            wasmtime: None,
            wine: None,
            verbose: false,
            prominent_compile_errors: false,
            jobs: None,
            maxrss: None,
            skip_oom_steps: false,
            incremental: None,
            target: None,
            cpu: None,
            dynamic_linker: None,
            optimize: None,
            options: vec![],
            reference_trace: None,
            no_reference_trace: false,
            build_file: None,
            cache_dir: None,
            global_cache_dir: None,
            zig_lib_dir: None,
            build_runner: None,
            seed: None,
            verbose_link: false,
            verbose_air: false,
            verbose_llvm_ir: None,
            verbose_llvm_bc: None,
            verbose_cimport: false,
            verbose_cc: false,
            verbose_llvm_cpu_features: false,
            env_cache: Default::default(),
        }
    }

    /// Sets the build step, this will default to `install` if not specified.
    pub fn step(&mut self, step: &str) -> &mut Self {
        self.step = Some(OsString::from(step));
        self
    }

    /// Sets the prefix path, this will default to `zig-out` if not specified.
    pub fn prefix(&mut self, prefix: impl AsRef<Path>) -> &mut Self {
        self.prefix = Some(env::current_dir().unwrap().join(prefix));
        self
    }

    /// Sets the prefix libraries path.
    pub fn prefix_lib_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self {
        self.prefix_lib_dir = Some(env::current_dir().unwrap().join(dir));
        self
    }

    /// Sets the prefix executables path.
    pub fn prefix_exe_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self {
        self.prefix_exe_dir = Some(env::current_dir().unwrap().join(dir));
        self
    }

    /// Sets the prefix headers path.
    pub fn prefix_include_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self {
        self.prefix_include_dir = Some(env::current_dir().unwrap().join(dir));
        self
    }

    /// Sets the preferred release mode.
    ///
    /// If set, the configuration will not inherit the optimization level of the current Rust
    /// profile.
    pub fn release(&mut self, release: ReleaseMode) -> &mut Self {
        self.release = Some(release);
        self
    }

    /// Sets the integration with Darling.
    pub fn darling(&mut self, enabled: bool) -> &mut Self {
        self.darling = Some(enabled);
        self
    }

    /// Sets the integration with QEMU.
    pub fn qemu(&mut self, enabled: bool) -> &mut Self {
        self.qemu = Some(enabled);
        self
    }

    /// Sets glibc runtime paths for the QEMU integration.
    pub fn glibc_runtimes(&mut self, path: impl AsRef<Path>) -> &mut Self {
        self.glibc_runtimes = Some(env::current_dir().unwrap().join(path));
        self
    }

    /// Sets whether to rely on Rosetta.
    pub fn rosetta(&mut self, enabled: bool) -> &mut Self {
        self.rosetta = Some(enabled);
        self
    }

    /// Sets the integration with wasmtime.
    pub fn wasmtime(&mut self, enabled: bool) -> &mut Self {
        self.wasmtime = Some(enabled);
        self
    }

    /// Sets the integration with Wine.
    pub fn wine(&mut self, enabled: bool) -> &mut Self {
        self.wine = Some(enabled);
        self
    }

    /// Enables verbose output.
    pub fn verbose(&mut self) -> &mut Self {
        self.verbose = true;
        self
    }

    /// Enables buffering of compile errors and display at the end.
    pub fn prominent_compile_errors(&mut self) -> &mut Self {
        self.prominent_compile_errors = true;
        self
    }

    /// Sets the limit of concurrent jobs.
    pub fn jobs(&mut self, jobs: usize) -> &mut Self {
        self.jobs = Some(jobs);
        self
    }

    /// Sets the memory usage limit.
    pub fn maxrss(&mut self, maxrss: usize) -> &mut Self {
        self.maxrss = Some(maxrss);
        self
    }

    /// Skips steps that exceed [`Build::maxrss`].
    pub fn skip_oom_steps(&mut self) -> &mut Self {
        self.skip_oom_steps = true;
        self
    }

    /// Sets the incremental compilation option.
    pub fn incremental(&mut self, enabled: bool) -> &mut Self {
        self.incremental = Some(enabled);
        self
    }

    /// Sets the CPU architecture, OS, and ABI to build for.
    ///
    /// If set, the configuration will not inherit the target and CPU features of the current Rust
    /// profile.
    pub fn target(&mut self, target: impl AsRef<OsStr>) -> &mut Self {
        self.target = Some(target.as_ref().into());
        self
    }

    /// Sets the CPU features for the build configuration.
    ///
    /// If set, the configuration will not inherit the CPU features of the current Rust profile.
    pub fn cpu(&mut self, cpu: impl AsRef<OsStr>) -> &mut Self {
        self.cpu = Some(cpu.as_ref().into());
        self
    }

    /// Adds a CPU feature from the build configuration.
    ///
    /// If set, the configuration will not inherit the CPU features of the current Rust profile.
    pub fn add_cpu(&mut self, cpu: impl AsRef<OsStr>) -> &mut Self {
        match self.cpu.as_mut() {
            None => {
                self.cpu = Some(cpu.as_ref().into());
            }
            Some(x) => {
                x.push("+");
                x.push(cpu);
            }
        }
        self
    }

    /// Subtracts a CPU feature from the build configuration.
    ///
    /// If set, the configuration will not inherit the CPU features of the current Rust profile.
    pub fn remove_cpu(&mut self, cpu: impl AsRef<OsStr>) -> &mut Self {
        match self.cpu.as_mut() {
            None => {
                self.cpu = Some(cpu.as_ref().into());
            }
            Some(x) => {
                x.push("-");
                x.push(cpu);
            }
        }
        self
    }

    /// Sets the path to the interpreter on the target system.
    pub fn dynamic_linker(&mut self, dynamic_linker: impl AsRef<Path>) -> &mut Self {
        self.dynamic_linker = Some(env::current_dir().unwrap().join(dynamic_linker));
        self
    }

    /// Sets the optimization modus of build configuration.
    ///
    /// If set, the configuration will not inherit the optimization level of the current Rust
    /// profile.
    pub fn optimize(&mut self, optimize: Optimize) -> &mut Self {
        self.optimize = Some(optimize);
        self
    }

    /// Adds the option `option` to the build configuration.
    ///
    /// # Panics
    ///
    /// Options must take the form `-Dfoo`.
    /// Additionally, it is not possible to specify any of the following options:
    /// - `-Dtarget=foo`: use [`Build::target`].
    /// - `-Dcpu=foo`: use [`Build::cpu`].
    /// - `-Ddynamic-linker=foo`: use [`Build::dynamic_linker`].
    /// - `-Doptimize=foo`: use [`Build::optimize`].
    pub fn option(&mut self, option: impl AsRef<OsStr>) -> &mut Self {
        let option = option.as_ref();
        let option_str = option.to_string_lossy();
        if !option_str.starts_with("-D") {
            panic!("invalid option: {}", option_str);
        }
        if option_str.starts_with("-Dtarget") {
            panic!("can not set target through an option: {}", option_str);
        }
        if option_str.starts_with("-Dcpu") {
            panic!("can not set cpu through an option: {}", option_str);
        }
        if option_str.starts_with("-Ddynamic-linker") {
            panic!(
                "can not set dynamic-linker through an option: {}",
                option_str
            );
        }
        if option_str.starts_with("-Doptimize") {
            panic!("can not set optimize through an option: {}", option_str);
        }

        self.options.push(option.into());
        self
    }

    /// Adds a list of options to the build configuration.
    ///
    /// # Panics
    ///
    /// See [`Build::option`] for the requirements of this method.
    pub fn options(&mut self, options: impl IntoIterator<Item = impl AsRef<OsStr>>) -> &mut Self {
        for option in options {
            self.option(option);
        }
        self
    }

    /// Sets the lines of reference trace to show per compile error.
    pub fn reference_trace(&mut self, reference_trace: usize) -> &mut Self {
        self.reference_trace = Some(reference_trace);
        self
    }

    /// Disables reference trace.
    pub fn no_reference_trace(&mut self) -> &mut Self {
        self.no_reference_trace = true;
        self
    }

    /// Sets the path to `build.zig`.
    pub fn build_file(&mut self, build_file: impl AsRef<Path>) -> &mut Self {
        self.build_file = Some(env::current_dir().unwrap().join(build_file));
        self
    }

    /// Sets the path to the local Zig cache directory.
    ///
    /// Defaults to `$OUT_DIR/.zig-cache`.
    pub fn cache_dir(&mut self, cache_dir: impl AsRef<Path>) -> &mut Self {
        self.cache_dir = Some(env::current_dir().unwrap().join(cache_dir));
        self
    }

    /// Sets the path to the global Zig cache directory.
    pub fn global_cache_dir(&mut self, cache_dir: impl AsRef<Path>) -> &mut Self {
        self.global_cache_dir = Some(env::current_dir().unwrap().join(cache_dir));
        self
    }

    /// Sets the path to the Zig lib directory.
    pub fn zig_lib_dir(&mut self, zig_lib_dir: impl AsRef<Path>) -> &mut Self {
        self.zig_lib_dir = Some(env::current_dir().unwrap().join(zig_lib_dir));
        self
    }

    /// Sets the path to the build runner.
    pub fn build_runner(&mut self, build_runner: impl AsRef<Path>) -> &mut Self {
        self.build_runner = Some(env::current_dir().unwrap().join(build_runner));
        self
    }

    /// Sets the build seed.
    pub fn seed(&mut self, seed: usize) -> &mut Self {
        self.seed = Some(seed);
        self
    }

    /// Enable compiler debug output for linking.
    pub fn verbose_link(&mut self) -> &mut Self {
        self.verbose_link = true;
        self
    }

    /// Enable compiler debug output for Zig AIR.
    pub fn verbose_air(&mut self) -> &mut Self {
        self.verbose_air = true;
        self
    }

    /// Enable compiler debug output for LLVM IR.
    pub fn verbose_llvm_ir(&mut self, file: impl AsRef<Path>) -> &mut Self {
        self.verbose_llvm_ir = Some(env::current_dir().unwrap().join(file));
        self
    }

    /// Enable compiler debug output for LLVM BC.
    pub fn verbose_llvm_bc(&mut self, file: impl AsRef<Path>) -> &mut Self {
        self.verbose_llvm_bc = Some(env::current_dir().unwrap().join(file));
        self
    }

    /// Enable compiler debug output for C imports.
    pub fn verbose_cimport(&mut self) -> &mut Self {
        self.verbose_cimport = true;
        self
    }

    /// Enable compiler debug output for C compilation.
    pub fn verbose_cc(&mut self) -> &mut Self {
        self.verbose_cc = true;
        self
    }

    /// Enable compiler debug output for LLVM CPU features.
    pub fn verbose_llvm_cpu_features(&mut self) -> &mut Self {
        self.verbose_llvm_cpu_features = true;
        self
    }

    /// Executes `zig build` command, compiling the library with all the configured options.
    pub fn build(&mut self) -> PathBuf {
        // Determine the prefix path if not specified.
        if self.prefix.is_none() {
            let mut prefix = PathBuf::from(getenv_unwrap("OUT_DIR"));
            prefix.push("zig-out");
            self.prefix(prefix);
        }

        // Determine the optimization level, if not specified.
        if self.release.is_none() && self.optimize.is_none() {
            let default_opt_level = match &getenv_unwrap("PROFILE")[..] {
                "debug" => Optimize::Debug,
                "release" | "bench" => Optimize::Default,
                unknown => {
                    eprintln!(
                        "Warning: unknown Rust profile={}; defaulting to a release build.",
                        unknown
                    );
                    Optimize::Default
                }
            };

            let opt_level = match &getenv_unwrap("OPT_LEVEL")[..] {
                "0" => Optimize::Debug,
                "1" | "2" | "3" => Optimize::ReleaseSafe,
                "s" | "z" => Optimize::ReleaseSmall,
                unknown => {
                    eprintln!(
                        "Warning: unknown opt-level={}; defaulting to a {:?} build.",
                        unknown, default_opt_level
                    );
                    default_opt_level
                }
            };

            if default_opt_level == Optimize::Default {
                self.release(ReleaseMode::Auto);
            }
            self.optimize(opt_level);
        }

        // Determine the target and CPU features, if not specified.
        if self.target.is_none() && self.cpu.is_none() {
            let (target, arch, _, _) = parse_target_triplet();
            self.target(target);

            let features = std::iter::once(&*arch)
                .chain(getenv_unwrap("CARGO_CFG_TARGET_FEATURE").split(','))
                .map(|feature| translate_arch_feature(&arch, feature))
                .collect::<Vec<_>>()
                .join("+");
            self.cpu(features);
        } else if self.target.is_none() {
            let (target, _, _, _) = parse_target_triplet();
            self.target(target);
        }

        // Determine the cache dir, if not set.
        if self.cache_dir.is_none() {
            let mut cache_dir = PathBuf::from(getenv_unwrap("OUT_DIR"));
            cache_dir.push(".zig-cache");
            self.cache_dir(cache_dir);
        }

        let mut cmd = Command::new(self.zig_executable());
        cmd.current_dir(&self.path);
        cmd.arg("build");

        // Configure step.
        if let Some(step) = &self.step {
            cmd.arg(step.clone());
        }

        // Configure general options.
        if let Some(prefix) = &self.prefix {
            cmd.arg("--prefix");
            cmd.arg(prefix.clone());
        }
        if let Some(prefix_lib_dir) = &self.prefix_lib_dir {
            cmd.arg("--prefix-lib-dir");
            cmd.arg(prefix_lib_dir.clone());
        }
        if let Some(prefix_exe_dir) = &self.prefix_exe_dir {
            cmd.arg("--prefix-exe-dir");
            cmd.arg(prefix_exe_dir.clone());
        }
        if let Some(prefix_include_dir) = &self.prefix_include_dir {
            cmd.arg("--prefix-include-dir");
            cmd.arg(prefix_include_dir.clone());
        }
        if let Some(release) = self.release {
            if release == ReleaseMode::Auto {
                cmd.arg("--release");
            } else {
                let release_string = match release {
                    ReleaseMode::Auto => unreachable!(),
                    ReleaseMode::Fast => "fast",
                    ReleaseMode::Safe => "safe",
                    ReleaseMode::Small => "small",
                };
                let arg = format!("--release={}", release_string);
                cmd.arg(arg);
            }
        }
        if let Some(darling) = self.darling {
            let arg = if darling { "-fdarling" } else { "-fno-darling" };
            cmd.arg(arg);
        }
        if let Some(qemu) = self.qemu {
            let arg = if qemu { "-fqemu" } else { "-fno-qemu" };
            cmd.arg(arg);
        }
        if let Some(glibc_runtimes) = &self.glibc_runtimes {
            cmd.arg("--glibc-runtimes");
            cmd.arg(glibc_runtimes);
        }
        if let Some(rosetta) = self.rosetta {
            let arg = if rosetta { "-frosetta" } else { "-fno-rosetta" };
            cmd.arg(arg);
        }
        if let Some(wasmtime) = self.wasmtime {
            let arg = if wasmtime {
                "-fwasmtime"
            } else {
                "-fno-wasmtime"
            };
            cmd.arg(arg);
        }
        if let Some(wine) = self.wine {
            let arg = if wine { "-fwine" } else { "-fno-wine" };
            cmd.arg(arg);
        }
        if self.verbose {
            cmd.arg("--verbose");
        }
        if self.prominent_compile_errors {
            cmd.arg("--prominent-compile-errors");
        }
        if let Some(jobs) = self.jobs {
            let arg = format!("-j{}", jobs);
            cmd.arg(arg);
        }
        if let Some(maxrss) = self.maxrss {
            cmd.arg("--maxrss");
            cmd.arg(maxrss.to_string());
        }
        if self.skip_oom_steps {
            cmd.arg("--skip-oom-steps");
        }
        if let Some(incremental) = self.incremental {
            let arg = if incremental {
                "-fincremental"
            } else {
                "-fno-incremental"
            };
            cmd.arg(arg);
        }

        // Configure project-specific options.
        if let Some(target) = &self.target {
            let arg = format!("-Dtarget={}", target.to_string_lossy());
            cmd.arg(arg);
        }
        if let Some(cpu) = &self.cpu {
            let arg = format!("-Dcpu={}", cpu.to_string_lossy());
            cmd.arg(arg);
        }
        if let Some(dynamic_linker) = &self.dynamic_linker {
            let arg = format!("-Ddynamic-linker={}", dynamic_linker.display());
            cmd.arg(arg);
        }
        if let Some(optimize) = &self.optimize {
            if optimize == &Optimize::Default {
                cmd.arg("-Doptimize");
            } else {
                let optimize_string = match optimize {
                    Optimize::Default => unreachable!(),
                    Optimize::Debug => "Debug",
                    Optimize::ReleaseSafe => "ReleaseSafe",
                    Optimize::ReleaseFast => "ReleaseFast",
                    Optimize::ReleaseSmall => "ReleaseSmall",
                };
                let arg = format!("-Doptimize={}", optimize_string);
                cmd.arg(arg);
            }
        }
        cmd.args(&self.options);

        // Configure advanced options.
        if let Some(reference_trace) = self.reference_trace {
            let arg = format!("-freference-trace={}", reference_trace);
            cmd.arg(arg);
        }
        if self.no_reference_trace {
            cmd.arg("-fno-reference-trace");
        }
        if let Some(build_file) = &self.build_file {
            cmd.arg("--build-file");
            cmd.arg(build_file.clone());
        }
        if let Some(cache_dir) = &self.cache_dir {
            cmd.arg("--cache-dir");
            cmd.arg(cache_dir.clone());
        }
        if let Some(global_cache_dir) = &self.global_cache_dir {
            cmd.arg("--global-cache-dir");
            cmd.arg(global_cache_dir.clone());
        }
        if let Some(zig_lib_dir) = &self.zig_lib_dir {
            cmd.arg("--zig-lib-dir");
            cmd.arg(zig_lib_dir.clone());
        }
        if let Some(build_runner) = &self.build_runner {
            cmd.arg("--build-runner");
            cmd.arg(build_runner.clone());
        }
        if let Some(seed) = self.seed {
            cmd.arg("--seed");
            cmd.arg(seed.to_string());
        }
        if self.verbose_link {
            cmd.arg("--verbose-link");
        }
        if self.verbose_air {
            cmd.arg("--verbose-air");
        }
        if let Some(verbose_llvm_ir) = &self.verbose_llvm_ir {
            let arg = format!("--verbose-llvm-ir={}", verbose_llvm_ir.display());
            cmd.arg(arg);
        }
        if let Some(verbose_llvm_bc) = &self.verbose_llvm_bc {
            let arg = format!("--verbose-llvm-bc={}", verbose_llvm_bc.display());
            cmd.arg(arg);
        }
        if self.verbose_cimport {
            cmd.arg("--verbose-cimport");
        }
        if self.verbose_cc {
            cmd.arg("--verbose-cc");
        }
        if self.verbose_llvm_cpu_features {
            cmd.arg("--verbose-llvm-cpu-features");
        }

        println!("running: {:?}", cmd);
        let status = match cmd.status() {
            Ok(status) => status,
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                fail(&format!(
                    "failed to execute command: {}\nis `zig` not installed?",
                    e
                ));
            }
            Err(e) => fail(&format!("failed to execute command: {}", e)),
        };
        if !status.success() {
            fail(&format!(
                "command did not execute successfully, got: {}",
                status
            ));
        }

        match &self.prefix {
            None => unreachable!(),
            Some(prefix) => prefix.clone(),
        }
    }
}

impl Build {
    fn zig_executable(&mut self) -> OsString {
        self.getenv_os("ZIG").unwrap_or("zig".into())
    }

    fn getenv_os(&mut self, v: &str) -> Option<OsString> {
        if let Some(val) = self.env_cache.get(v) {
            return val.clone();
        }
        let r = env::var_os(v);
        println!("{} = {:?}", v, r);
        self.env_cache.insert(v.to_string(), r.clone());
        r
    }
}

/// Builds the native library rooted at `path` with the default zig options.
/// This will return the directory in which the library was installed.
///
/// # Examples
///
/// ```no_run
/// use zigcli;
///
/// // Builds the project in the directory located in `libfoo`, installing it
/// // into $OUT_DIR
/// let dst = zigcli::build("libfoo");
/// let dst_lib = dst.join("lib");
///
/// println!("cargo:rustc-link-search=native={}", dst_lib.display());
/// println!("cargo:rustc-link-lib=static=foo");
/// ```
pub fn build(path: impl AsRef<Path>) -> PathBuf {
    Build::new(path.as_ref()).build()
}

fn getenv_unwrap(v: &str) -> String {
    match env::var(v) {
        Ok(s) => s,
        Err(..) => fail(&format!("environment variable `{}` not defined", v)),
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild failed, must exit now", s)
}

fn parse_target_triplet() -> (String, String, String, Option<String>) {
    // Read the target from the environment variables.
    let arch = getenv_unwrap("CARGO_CFG_TARGET_ARCH");
    let sys = getenv_unwrap("CARGO_CFG_TARGET_OS");
    let env = getenv_unwrap("CARGO_CFG_TARGET_ENV");
    let abi = getenv_unwrap("CARGO_CFG_TARGET_ABI");

    // The abi is composed of env and abi.
    let abi = format!("{env}{abi}");

    let (triplet, abi) = if abi.is_empty() {
        (format!("{arch}-{sys}"), None)
    } else {
        (format!("{arch}-{sys}-{abi}"), Some(abi))
    };

    (triplet, arch, sys, abi)
}

fn translate_arch_feature(arch: &str, feature: &str) -> String {
    let feature = feature.replace("-", "_").replace(".", "_");
    match arch {
        target if target.starts_with("x86") => translate_x86_target_feature(feature),
        _ => feature,
    }
}

fn translate_x86_target_feature(feature: String) -> String {
    match &*feature {
        "avx512vbmi1" => "avx512vbmi".to_string(),
        "bmi1" => "bmi".to_string(),
        "cmpxchg16b" => "cx16".to_string(),
        "rdrand" => "rdrnd".to_string(),
        "lahfsahf" => "sahf".to_string(),
        "pclmulqdq" => "pclmul".to_string(),
        _ => feature,
    }
}

/// Runtime11 Build Script
///
/// The different executables that make up a runtime must be linked without
/// the standard runtime on the system. There are several approaches to build
/// a runtime as rust crate:
///
/// 1) Create a custom rustc-target definition for each supported platform
///    and ensure this target uses the correct compiler and linker flags. The
///    downside is that this will easily get out-of-sync with the official
///    platform targets shipped with rustc. Furthermore, this requires a
///    nightly compiler.
///
/// 2) Use the official platform targets but pass suitable flags to the linker.
///    This requires inside knowledge of the individual targets, but avoids
///    major deviation from the target definition. Furthermore, this can be
///    achieved on a stable compiler.
///
/// We use the second approach, since the existing linux targets are fairly
/// straightforward to work with. In most cases, all we need is to pass
/// `-nostdlib` and `--no-dynamic-linker`.
///
/// Apart from the linker settings, we also need to adjust the rust standard
/// library settings. In particular, unwinding is not supported. This means
/// `-C panic=abort` is required as panic strategy. Sadly, cargo does not allow
/// to specify this as a per-binary setting, but only per crate. Hence, it is
/// up to the caller to pick a suitable cargo-profile. The cargo workspace
/// configuration of this project picks `panic=abort` as default for the
/// development and release profiles. However, these are only applied if you
/// compile the binaries from a source checkout. Unfortunately, no suitable
/// replacement option is currently available on stable cargo.

fn main() {
    // rt11-loader:
    //
    // Pass `-nostdlib` to compiler and linker to ensure neither `libc`, nor
    // any `crt.o`+friends, nor compiler builtins are linked.
    //
    // Funnel `--no-dynamic-linker` to the linker to ensure `PT_INTERP` is
    // omitted from the resulting binary.
    println!("cargo:rustc-link-arg-bin=rt11-loader=-nostdlib");
    println!("cargo:rustc-link-arg-bin=rt11-loader=-Wl,--no-dynamic-linker");
}

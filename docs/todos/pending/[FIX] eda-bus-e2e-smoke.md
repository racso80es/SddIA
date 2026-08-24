Refinar pbi fix con el siguiente error al pasar test de PR wn github test eda-bus-e2e-smoke

warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /home/runner/work/SddIA/SddIA/SddIA/tools/wasi-poc/Cargo.toml
workspace: /home/runner/work/SddIA/SddIA/SddIA/Cargo.toml
    Updating crates.io index
 Downloading crates ...
  Downloaded bufstream v0.1.4
  Downloaded arrayvec v0.5.2
  Downloaded base64 v0.13.1
  Downloaded bitflags v1.3.2
  Downloaded bitflags v2.11.1
  Downloaded cfg_aliases v0.2.2
  Downloaded native-tls v0.2.18
  Downloaded foreign-types v0.3.2
  Downloaded ctrlc v3.5.2
  Downloaded pkg-config v0.3.34
  Downloaded imap-proto v0.10.2
  Downloaded openssl-macros v0.1.1
  Downloaded foreign-types-shared v0.1.1
  Downloaded openssl-probe v0.2.1
  Downloaded static_assertions v1.1.0
  Downloaded imap v2.4.1
  Downloaded openssl-sys v0.9.117
  Downloaded nom v5.1.3
  Downloaded vcpkg v0.2.15
  Downloaded openssl v0.10.81
  Downloaded nix v0.31.3
  Downloaded lexical-core v0.7.6
   Compiling proc-macro2 v1.0.106
   Compiling quote v1.0.45
   Compiling unicode-ident v1.0.24
   Compiling cfg-if v1.0.4
   Compiling serde_core v1.0.228
   Compiling memchr v2.8.1
   Compiling serde v1.0.228
   Compiling zmij v1.0.21
   Compiling serde_json v1.0.150
   Compiling itoa v1.0.18
   Compiling getrandom v0.4.2
   Compiling uuid v1.23.2
   Compiling syn v2.0.117
   Compiling autocfg v1.5.1
   Compiling version_check v0.9.5
  Could not find openssl via pkg-config:
  pkg-config has not been configured to support cross-compilation.

  Install a sysroot for the target platform and configure it via
  PKG_CONFIG_SYSROOT_DIR and PKG_CONFIG_PATH, or install a
  cross-compiling wrapper for pkg-config and set it via
  PKG_CONFIG environment variable.

  cargo:warning=Could not find directory of OpenSSL installation, and this `-sys` crate cannot proceed without this knowledge. If OpenSSL is installed and this crate had trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the compilation process. See stderr section below for further information.

  --- stderr


  Could not find directory of OpenSSL installation, and this `-sys` crate cannot
  proceed without this knowledge. If OpenSSL is installed and this crate had
  trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
  compilation process.

  Make sure you also have the development packages of openssl installed.
  For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.

  If you're in a situation where you think the directory *should* be found
  automatically, please open a bug at https://github.com/rust-openssl/rust-openssl
  and include information about your system as well as this message.

  $HOST = x86_64-unknown-linux-gnu
  $TARGET = wasm32-wasip1
  openssl-sys = 0.9.117


warning: build failed, waiting for other jobs to finish...
Error: Process completed with exit code 101.
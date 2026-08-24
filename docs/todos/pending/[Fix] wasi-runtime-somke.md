Refinar pbi fix con el siguiente error al pasar test de PR wn github test wasi-runtime-somke

19s
Run cd SddIA && cargo build --workspace --target wasm32-wasip1
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   /home/runner/work/SddIA/SddIA/SddIA/tools/wasi-poc/Cargo.toml
workspace: /home/runner/work/SddIA/SddIA/SddIA/Cargo.toml
    Updating crates.io index
 Downloading crates ...
  Downloaded foreign-types v0.3.2
  Downloaded openssl-sys v0.9.117
  Downloaded base64 v0.13.1
  Downloaded arrayvec v0.5.2
  Downloaded bitflags v2.11.1
  Downloaded bufstream v0.1.4
  Downloaded cfg_aliases v0.2.2
  Downloaded foreign-types-shared v0.1.1
  Downloaded bitflags v1.3.2
  Downloaded openssl-macros v0.1.1
  Downloaded static_assertions v1.1.0
  Downloaded imap v2.4.1
  Downloaded vcpkg v0.2.15
  Downloaded ctrlc v3.5.2
  Downloaded imap-proto v0.10.2
  Downloaded pkg-config v0.3.34
  Downloaded native-tls v0.2.18
  Downloaded openssl-probe v0.2.1
  Downloaded nom v5.1.3
  Downloaded nix v0.31.3
  Downloaded openssl v0.10.81
  Downloaded lexical-core v0.7.6
   Compiling proc-macro2 v1.0.106
   Compiling quote v1.0.45
   Compiling unicode-ident v1.0.24
   Compiling cfg-if v1.0.4
   Compiling serde_core v1.0.228
   Compiling serde v1.0.228
   Compiling memchr v2.8.1
   Compiling zmij v1.0.21
   Compiling serde_json v1.0.150
   Compiling itoa v1.0.18
   Compiling syn v2.0.117
   Compiling getrandom v0.4.2
   Compiling autocfg v1.5.1
   Compiling uuid v1.23.2
   Compiling version_check v0.9.5
   Compiling num-traits v0.2.19
   Compiling stable_deref_trait v1.2.1
   Compiling shlex v2.0.1
   Compiling find-msvc-tools v0.1.9
   Compiling cc v1.2.63
   Compiling typenum v1.20.1
   Compiling writeable v0.6.3
   Compiling litemap v0.8.2
   Compiling icu_properties_data v2.2.0
   Compiling utf8_iter v1.0.4
   Compiling icu_normalizer_data v2.2.0
   Compiling smallvec v1.15.2
   Compiling once_cell v1.21.4
   Compiling percent-encoding v2.3.2
   Compiling generic-array v0.14.7
   Compiling libc v0.2.186
   Compiling log v0.4.30
   Compiling synstructure v0.13.2
   Compiling ring v0.17.14
   Compiling aho-corasick v1.1.4
   Compiling regex-syntax v0.8.10
   Compiling zeroize v1.9.0
   Compiling rustls-pki-types v1.14.1
   Compiling regex-automata v0.4.14
   Compiling serde_derive v1.0.228
   Compiling zerofrom-derive v0.1.7
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.2
   Compiling sddia-io v0.1.0 (/home/runner/work/SddIA/SddIA/SddIA/sddia-io)
   Compiling yoke v0.8.3
   Compiling zerovec-derive v0.11.3
   Compiling displaydoc v0.2.6
   Compiling chrono v0.4.44
   Compiling zerotrie v0.2.4
   Compiling wasi v0.11.1+wasi-snapshot-preview1
   Compiling zerovec v0.11.6
   Compiling base64 v0.22.1
   Compiling getrandom v0.2.17
   Compiling tinystr v0.8.3
   Compiling potential_utf v0.1.5
   Compiling icu_collections v2.2.0
   Compiling icu_locale_core v2.2.0
   Compiling hybrid-array v0.4.12
   Compiling num-integer v0.1.46
   Compiling untrusted v0.9.0
   Compiling ryu v1.0.23
   Compiling icu_provider v2.2.0
   Compiling pkg-config v0.3.34
   Compiling icu_normalizer v2.2.0
   Compiling icu_properties v2.2.0
   Compiling crc32fast v1.5.0
   Compiling vcpkg v0.2.15
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling syn v3.0.3
   Compiling idna_adapter v1.2.2
   Compiling idna v1.1.0
   Compiling openssl-sys v0.9.117
   Compiling getrandom v0.3.4
   Compiling zerocopy v0.8.55
   Compiling ref-cast v1.0.26
   Compiling adler2 v2.0.1
   Compiling rustls v0.23.40
   Compiling simd-adler32 v0.3.9
   Compiling hex v0.4.3
   Compiling miniz_oxide v0.8.9
warning: openssl-sys@0.9.117: Could not find directory of OpenSSL installation, and this `-sys` crate cannot proceed without this knowledge. If OpenSSL is installed and this crate had trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the compilation process. See stderr section below for further information.
error: failed to run custom build command for `openssl-sys v0.9.117`

Caused by:
  process didn't exit successfully: `/home/runner/work/SddIA/SddIA/SddIA/target/debug/build/openssl-sys-71aa3fd62e7b3785/build-script-main` (exit status: 101)
  --- stdout
  cargo:rustc-check-cfg=cfg(osslconf, values("OPENSSL_NO_OCB", "OPENSSL_NO_SM4", "OPENSSL_NO_SEED", "OPENSSL_NO_CHACHA", "OPENSSL_NO_CAST", "OPENSSL_NO_IDEA", "OPENSSL_NO_CAMELLIA", "OPENSSL_NO_RC4", "OPENSSL_NO_BF", "OPENSSL_NO_PSK", "OPENSSL_NO_DEPRECATED_3_0", "OPENSSL_NO_SCRYPT", "OPENSSL_NO_SM3", "OPENSSL_NO_RMD160", "OPENSSL_NO_EC2M", "OPENSSL_NO_OCSP", "OPENSSL_NO_CMS", "OPENSSL_NO_COMP", "OPENSSL_NO_SOCK", "OPENSSL_NO_STDIO", "OPENSSL_NO_EC", "OPENSSL_NO_SSL3_METHOD", "OPENSSL_NO_KRB5", "OPENSSL_NO_TLSEXT", "OPENSSL_NO_SRP", "OPENSSL_NO_SRTP", "OPENSSL_NO_RFC3779", "OPENSSL_NO_SHA", "OPENSSL_NO_NEXTPROTONEG", "OPENSSL_NO_ENGINE", "OPENSSL_NO_BUF_FREELISTS", "OPENSSL_NO_RC2"))
  cargo:rustc-check-cfg=cfg(openssl)
  cargo:rustc-check-cfg=cfg(libressl)
  cargo:rustc-check-cfg=cfg(boringssl)
  cargo:rustc-check-cfg=cfg(awslc)
  cargo:rustc-check-cfg=cfg(awslc_pregenerated)
  cargo:rustc-check-cfg=cfg(libressl250)
  cargo:rustc-check-cfg=cfg(libressl251)
  cargo:rustc-check-cfg=cfg(libressl252)
  cargo:rustc-check-cfg=cfg(libressl261)
  cargo:rustc-check-cfg=cfg(libressl270)
  cargo:rustc-check-cfg=cfg(libressl271)
  cargo:rustc-check-cfg=cfg(libressl273)
  cargo:rustc-check-cfg=cfg(libressl280)
  cargo:rustc-check-cfg=cfg(libressl281)
  cargo:rustc-check-cfg=cfg(libressl291)
  cargo:rustc-check-cfg=cfg(libressl310)
  cargo:rustc-check-cfg=cfg(libressl321)
  cargo:rustc-check-cfg=cfg(libressl332)
  cargo:rustc-check-cfg=cfg(libressl340)
  cargo:rustc-check-cfg=cfg(libressl350)
  cargo:rustc-check-cfg=cfg(libressl360)
  cargo:rustc-check-cfg=cfg(libressl361)
  cargo:rustc-check-cfg=cfg(libressl370)
  cargo:rustc-check-cfg=cfg(libressl380)
  cargo:rustc-check-cfg=cfg(libressl381)
  cargo:rustc-check-cfg=cfg(libressl382)
  cargo:rustc-check-cfg=cfg(libressl390)
  cargo:rustc-check-cfg=cfg(libressl400)
  cargo:rustc-check-cfg=cfg(libressl410)
  cargo:rustc-check-cfg=cfg(libressl420)
  cargo:rustc-check-cfg=cfg(libressl430)
  cargo:rustc-check-cfg=cfg(ossl101)
  cargo:rustc-check-cfg=cfg(ossl102)
  cargo:rustc-check-cfg=cfg(ossl102f)
  cargo:rustc-check-cfg=cfg(ossl102h)
  cargo:rustc-check-cfg=cfg(ossl110)
  cargo:rustc-check-cfg=cfg(ossl110f)
  cargo:rustc-check-cfg=cfg(ossl110g)
  cargo:rustc-check-cfg=cfg(ossl110h)
  cargo:rustc-check-cfg=cfg(ossl111)
  cargo:rustc-check-cfg=cfg(ossl111b)
  cargo:rustc-check-cfg=cfg(ossl111c)
  cargo:rustc-check-cfg=cfg(ossl111d)
  cargo:rustc-check-cfg=cfg(ossl300)
  cargo:rustc-check-cfg=cfg(ossl310)
  cargo:rustc-check-cfg=cfg(ossl320)
  cargo:rustc-check-cfg=cfg(ossl330)
  cargo:rustc-check-cfg=cfg(ossl340)
  cargo:rustc-check-cfg=cfg(ossl400)
  cargo:rerun-if-env-changed=WASM32_WASIP1_OPENSSL_LIB_DIR
  WASM32_WASIP1_OPENSSL_LIB_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
  OPENSSL_LIB_DIR unset
  cargo:rerun-if-env-changed=WASM32_WASIP1_OPENSSL_INCLUDE_DIR
  WASM32_WASIP1_OPENSSL_INCLUDE_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
  OPENSSL_INCLUDE_DIR unset
  cargo:rerun-if-env-changed=WASM32_WASIP1_OPENSSL_DIR
  WASM32_WASIP1_OPENSSL_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_DIR
  OPENSSL_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS_wasm32-wasip1
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS_wasm32_wasip1
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG_ALLOW_CROSS
  cargo:rerun-if-env-changed=PKG_CONFIG_ALLOW_CROSS
  cargo:rerun-if-env-changed=PKG_CONFIG_wasm32-wasip1
  cargo:rerun-if-env-changed=PKG_CONFIG_wasm32_wasip1
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_wasm32-wasip1
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_wasm32_wasip1
  cargo:rerun-if-env-changed=TARGET_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR


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
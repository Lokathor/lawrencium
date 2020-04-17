[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/msvrnnl40fhs55tq/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/lawrencium/branch/master)
[![crates.io](https://img.shields.io/crates/v/lawrencium.svg)](https://crates.io/crates/lawrencium)
[![docs.rs](https://docs.rs/lawrencium/badge.svg)](https://docs.rs/lawrencium/)

# lawrencium

Bindings to a limited subset of the Windows API.

These aren't any better than the [winapi](https://docs.rs/winapi) crate, which
you _should_ use for general purpose Windows API usage. The difference is that
this focuses on a fairly minimal subset of the API so that the crate can build
faster. Not that `winapi` builds that slow to begin with.

Okay, I'll admit it, we're kinda just doing it for fun, oh well.

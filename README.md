# dockurl

[![Build Status](https://github.com/TechEmpower/dockurl/workflows/build/badge.svg?branch=master&event=push)](https://github.com/TechEmpower/dockurl/actions?query=workflow%3Abuild+branch%3Amaster)

**WARNING**: This library was written for the explicit use of 
[TFBToolset](https://github.com/techempower/TFBToolset) and should
**NOT** be used in any other production setting. Much of `dockurl` is simply
`todo`s at present, and may never be implemented.

`dockurl` is a synchronous low-level 
[Docker API](https://docs.docker.com/engine/api/v1.40/) Rust library. It relies
upon the [curl](https://crates.io/crates/curl) Rust library. As such, 
`dockurl` works on any platform with `libcurl`. 

## Authors

* **Mike Smith** - *Initial work* - [msmith-techempower](https://github.com/msmith-techempower)

## License

This project is licensed under the BSD-3-Clause License - see the [LICENSE](LICENSE) file for details

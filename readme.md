# recrep

Generate crash reports with ease.

_recrep_ uses the AppCenter API to compile a crash report for a given organization, application and optional version. When no version is specified, the latest version is used.

## How To

It is easiest to run _recrep_ with cargo. Clone this repository and prefix your commands with `cargo run --`.

```shell
$ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/recrep --help`
recrep 0.1.0
The Mobile Releases Team of XING SE <mobile_releases@xing.com>
Recycled Crep: Look at your crashes.

USAGE:
    recrep [OPTIONS] --application <application> --organization <organization> --token <token>

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -a, --application <application>
    -c, --organization <organization>
    -t, --token <token>                  The AppCenter API token [env: RECREP_APPCENTER_API_TOKEN=]
    -v, --version <version>
```

The **token** can be provided as environment variable: `RECREP_APPCENTER_API_TOKEN`.

**Example:**

This will retrieve the crashes for the latest available version for the given organization. If you want to retrieve crashes of a certain version you may provide it using an additional parameter:  `--version 1.2.3`.

```shell
$ cargo run -- --organization XING-SE-Organization --application XING
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/recrep --organization XING-SE-Organization --application XING`

Hello everyone!

This is the crash newsletter of v7.41.3 💌


First appeared on 2019-05-17T04:03:49Z and occurred 329 times in 7.41.3/10167 and affected 330 devices.
File:    NSManagedObjectContext+XNGManagedObjectContextObservable.m
Class:   XNGManagedObjectContextObservable
Method:  notifyObserversWithContext:changeInfo:
More on AppCenter: https://appcenter.ms/orgs/XING-SE-Organization/apps/XING/crashes/errors/1005734617u/overview.

-------------------------------

First appeared on 2019-05-16T18:35:54Z and occurred 123 times in 7.41.3/...
```

## Test

You can execute all tests by running `cargo test`.

## Build

The Jenkins pipelines which use the build are located in the [mobile-releases/crash-reports]([crash-reports](https://source.xing.com/mobile-releases/crash-reports)) repository. Since those jobs run on Linux nodes we need to cross compile for Linux. There is no automatic deploymentbuild process yet, hence we have to do it by hand:

[muslrust](https://github.com/clux/muslrust) allows us to build Rust binaries using the [musl](https://www.musl-libc.org) toolchain.

- muslrust is dockerized, so start Docker
- build the app:

```bash
docker pull clux/muslrust
docker run -v $PWD:/volume --rm -t clux/muslrust cargo build --release
```

- this builds a release build of recrep and places it to `./target/x86_64-unknown-linux-musl/release/`
- copy the `recrep` executable to whereever it will be executed

## Documentation

You can generate and open the documentation by running `cargo doc --open`

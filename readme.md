# recrep

Generate crash reports with ease.

_recrep_ uses the AppCenter API to compile a crash report for a given organization, application and optional version. When no version is specified, the latest version is used.

## How To

The easiest way to run _recrep_ is using cargo. Clone this repository and prefix your commands with `cargo run --`.

```shell
$ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/recrep --help`
recrep 0.1.1
The Mobile Releases Team of XING SE <mobile_releases@xing.com>
Recycled Crep: Look at your crashes.

USAGE:
    recrep [OPTIONS] --application <application> --organization <organization> --token <token>

FLAGS:
    --arithmetic-mean    Use the arithmetic mean as threshold value, and only show crashes exceeding this value.
                         This flag is incompatible with `--threshold`.
    -h, --help           Prints help information

OPTIONS:
    -a, --application <application>      The application identifier as seen in AppCenter urls.
    -g, --group <distribution-group>     Distribution group used to search for the latest version released into this
                                         distribution group.
    -c, --organization <organization>    The organization the app belongs to.
    -o, --outfile <outfile>              An optional filename to write the report to instead of printing it to the
                                         console.
    -m, --threshold <threshold>          Set a threshold value determining a maximum amount of crashes and show a
                                         percentage of how many percent are reached for each crash in the crash list.
    -t, --token <token>                  The AppCenter API token [env: RECREP_APPCENTER_API_TOKEN=]
    -v, --version <version>              The app version. If none is specified, the latest available version will be
                                         picked - be aware that the latest version might not have crashes yet.
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

[muslrust](https://github.com/clux/muslrust) allows us to build Rust binaries using the [musl](https://www.musl-libc.org) toolchain, which is nice if you want to cross-compile for Linux.

- muslrust is dockerized, so start [Docker](https://www.docker.com)
- build the app:

```bash
docker pull clux/muslrust
docker run -v $PWD:/volume --rm -t clux/muslrust cargo build --release
```

- this builds a release build of recrep and places it to `./target/x86_64-unknown-linux-musl/release/`
- copy the `recrep` executable to whereever it will be executed

## Documentation

You can generate and open the documentation by running `cargo doc --open`

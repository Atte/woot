# woot-cli

An **unofficial** command line client for Wooting keyboards.

Only properly tested on Windows with a 60HE, but _should_ work on all Lekker based Wootings on all major platforms.

## Probably supported keyboards

- Wooting Two Lekker Edition
- Wooting Two HE
- Wooting 60HE

## Probably supported operating systems

- Windows
- Linux
- BSD etc.
- MacOS

## How to use?

1. `git clone`
2. `cargo install --path .`
3. `woot --help`

## Autoswitch config format

The first matching rule is applied.

If no rules match, the profile is not changed, so you might want to add a rule without filters at the end.

```toml
[[rules]]
name = "Overwatch"
profile = 3

[[rules]]
cmd = ["java", "-jar", "C:/Games/minecraft.jar"]
profile = 2

[[rules]]
name = "Program Needing Multiple Profiles"

[[rules]]
exe = "C:/Games/CounterStrike/CS.exe"
profile = 3

[[rules]]
profile = 0
```

![crab pet](https://i.imgur.com/LbZJgmm.gif) 

# rustlings 🦀❤️

Greetings and welcome to `rustlings`. This project contains small exercises to get you used to reading and writing Rust code. This includes reading and responding to compiler messages!

_...looking for the old, web-based version of Rustlings? Try [here](https://github.com/rust-lang/rustlings/tree/rustlings-1)_

Alternatively, for a first-time Rust learner, there's several other resources:

- [The Book](https://doc.rust-lang.org/book/index.html) - The most comprehensive resource for learning Rust, but a bit theoretical sometimes. You will be using this along with Rustlings!
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - Learn Rust by solving little exercises! It's almost like `rustlings`, but online

## Getting Started

_Note: If you're on MacOS, make sure you've installed Xcode and its developer tools by typing `xcode-select --install`._

_Note: If you have Xcode 10+ installed, you also need to install the package file found at `/Library/Developer/CommandLineTools/Packages/macOS_SDK_headers_for_macOS_10.14.pkg`._

You will need to have Rust installed. You can get it by visiting https://rustup.rs. This'll also install Cargo, Rust's package/project manager.

## MacOS/Linux

Just run:

```bash
curl -L https://git.io/rustlings | bash
# Or if you want it to be installed to a different path:
curl -L https://git.io/rustlings | bash -s mypath/
```

This will install Rustlings and give you access to the `rustlings` command. Run it to get started!

## Manually

Basically: Clone the repository, checkout to the latest tag, run `cargo install`.

```bash
git clone https://github.com/rust-lang/rustlings
cd rustlings
git checkout tags/1.0.0 # or whatever the latest version is (find out at https://github.com/rust-lang/rustlings/releases/latest)
cargo install --force --path .
```

Same as above, run `rustlings` to get started.

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend that you have a look at them before you start. 

The task is simple. Most exercises contain an error that keep it from compiling, and it's up to you to fix it! Some exercises are also ran as tests, but rustlings handles them all the same. To run the exercises in the recommended order, execute:

```bash
rustlings watch
```

This will try to verify the completion of every exercise in a predetermined order (what we think is best for newcomers). It will also rerun automatically every time you change a file in the `exercises/` directory. If you want to only run it once, you can use:

```bash
rustlings verify
```

This will do the same as watch, but it'll quit after running.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

```bash
rustlings run exercises/path/to/exercise.rs
```

In case you get stuck, there is usually a hint at the bottom of each exercise.

## Testing yourself

After every couple of sections, there will be a test that'll test your knowledge on a bunch of sections at once. These tests are found in `exercises/testN.rs`.

## Completion

Rustlings isn't done; there are a couple of sections that are very experimental and don't have proper documentation. These include:

- Errors (`exercises/errors/`)
- Option (`exercises/option/`)
- Result (`exercises/result/`)
- Move Semantics (could still be improved, `exercises/move_semantics/`)

Additionally, we could use exercises on a couple of topics:

- Structs
- Better ownership stuff
- `impl`
- ??? probably more

If you are interested in improving or adding new ones, please feel free to contribute! Read on for more information :)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Credits

`rustlings` was originally written by [Carol](https://github.com/carols10cents)!

## Results
```bash
PS> rustlings.exe watch
✓ Successfully compiled exercises/variables/variables1.rs!
✓ Successfully compiled exercises/variables/variables2.rs!
✓ Successfully compiled exercises/variables/variables3.rs!
✓ Successfully compiled exercises/variables/variables4.rs!
✓ Successfully tested exercises/if/if1.rs!
✓ Successfully compiled exercises/functions/functions1.rs!
✓ Successfully compiled exercises/functions/functions2.rs!
✓ Successfully compiled exercises/functions/functions3.rs!
✓ Successfully compiled exercises/functions/functions4.rs!
✓ Successfully compiled exercises/functions/functions5.rs!
✓ Successfully tested exercises/test1.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types1.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types2.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types3.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types4.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types5.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types6.rs!
✓ Successfully tested exercises/structs/structs1.rs!
✓ Successfully tested exercises/tests/tests1.rs!
✓ Successfully tested exercises/tests/tests2.rs!
✓ Successfully tested exercises/tests/tests3.rs!
✓ Successfully tested exercises/test2.rs!
✓ Successfully compiled exercises/strings/strings1.rs!
✓ Successfully compiled exercises/strings/strings2.rs!
✓ Successfully compiled exercises/test3.rs!
✓ Successfully compiled exercises/modules/modules1.rs!
✓ Successfully compiled exercises/modules/modules2.rs!
✓ Successfully compiled exercises/macros/macros1.rs!
✓ Successfully compiled exercises/macros/macros2.rs!
✓ Successfully compiled exercises/macros/macros3.rs!
✓ Successfully compiled exercises/macros/macros4.rs!
✓ Successfully compiled exercises/test4.rs!
✓ Successfully compiled exercises/move_semantics/move_semantics1.rs!
✓ Successfully compiled exercises/move_semantics/move_semantics2.rs!
✓ Successfully compiled exercises/move_semantics/move_semantics3.rs!
✓ Successfully compiled exercises/move_semantics/move_semantics4.rs!
✓ Successfully tested exercises/error_handling/errors1.rs!
✓ Successfully tested exercises/error_handling/errors2.rs!
✓ Successfully tested exercises/error_handling/errors3.rs!
✓ Successfully tested exercises/error_handling/errorsn.rs!
✓ Successfully compiled exercises/error_handling/option1.rs!
✓ Successfully tested exercises/error_handling/result1.rs!
✓ Successfully compiled exercises/threads/threads1.rs!
✓ Successfully compiled exercises/standard_library_types/arc1.rs!
✓ Successfully tested exercises/standard_library_types/iterators2.rs!
✓ Successfully tested exercises/standard_library_types/iterators3.rs!
✓ Successfully tested exercises/standard_library_types/iterators4.rs!
```
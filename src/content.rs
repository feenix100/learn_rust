//! Educational content source.
//!
//! All learning text lives here so UI files can focus on rendering and
//! interaction. This separation is useful in real apps and easy to reason about.

use crate::models::{Concept, ConceptId};

pub fn ordered_concepts() -> Vec<Concept> {
    vec![
        concept_what_is_rust(),
        concept_variables(),
        concept_functions(),
        concept_types(),
        concept_control_flow(),
        concept_ownership(),
        concept_borrowing_refs(),
        concept_slices(),
        concept_structs(),
        concept_enums(),
        concept_pattern_matching(),
        concept_option_result(),
        concept_vectors_strings(),
        concept_error_handling(),
        concept_modules(),
        concept_traits_generics(),
        concept_lifetimes(),
        concept_cargo(),
    ]
}

fn concept_what_is_rust() -> Concept {
    Concept {
        id: ConceptId::WhatIsRust,
        title: "What Is The Rust Programming Language?",
        why_it_matters: "Rust gives memory safety, speed, and reliability without a garbage collector.",
        explanation: "The Rust programming language is a general-purpose, compiled, and statically-typed language that emphasizes performance, memory safety, and secure concurrency. It is designed to provide low-level control similar to C and C++ but with a strong borrow checker that catches common memory-related bugs at compile time without a runtime garbage collector. It is used for command-line tools, back-end services, game engines, browser components, and embedded systems.\n\nWhat makes Rust stand out is its compile-time safety model: ownership, borrowing, and lifetimes help the compiler catch memory bugs before your program runs. That means fewer crashes in production and more confidence when refactoring.\n\nWhy Rust needs to be installed first:\n- The Rust compiler (`rustc`) turns your `.rs` source code into real executables\n- Cargo (Rust's tool) manages builds, dependencies, checks, and tests\n- Tooling like `clippy` and `rustfmt` gives fast feedback and clean code style\n\nHow to install (beginner path):\n- Go to `https://rustup.rs` and install with Rustup (official installer)\n- Rustup installs `rustc`, `cargo`, and standard tools in one step\n- Verify with `rustc --version` and `cargo --version`\n\nIn practice, Rust aims for three outcomes at once:\n- Performance close to C/C++\n- Safety against common memory and concurrency bugs\n- Strong tooling (Cargo, clippy, rustfmt, docs) that supports maintainable codebases",
        code: r#"fn main() {
    println!("Hello, Rust learner!");
}"#,
        code_explanation: "Every executable Rust program starts at `main`. `println!` is a macro (note the `!`) that writes formatted output to the console.",
        beginner_mistake: "Forgetting the `!` on macros, e.g. writing `println(\"x\")` instead of `println!(\"x\")`.",
        recap: "Rust combines safety, speed, and strong tooling. Learn the core rules once, then let the compiler enforce them consistently.",
    }
}

fn concept_variables() -> Concept {
    Concept {
        id: ConceptId::VariablesMutability,
        title: "Variables and Mutability",
        why_it_matters: "Rust makes state changes explicit, which reduces bugs and makes code easier to reason about.",
        explanation: "In Rust, variables are immutable by default. That means once you bind a value with `let`, it cannot change unless you opt in with `mut`.\n\nThis default is intentional: it helps you separate values that are stable from values that are expected to change. As code grows, that distinction makes debugging and refactoring much easier.\n\nA useful beginner rule:\n- Start with `let`\n- Add `mut` only when the compiler tells you the value must change\n\nThis creates safer, more predictable code because mutation is visible at the exact point where it is allowed.",
        code: r#"let score = 10;
let mut level = 1;
level += 1;"#,
        code_explanation: "`score` is read-only after creation. `level` is writable because `mut` marks it as mutable, so `level += 1` is allowed.",
        beginner_mistake: "Adding `mut` to every variable automatically, which hides where real state changes happen.",
        recap: "Immutability is the default. Treat `mut` as an explicit signal that state is meant to change.",
    }
}

fn concept_functions() -> Concept {
    Concept {
        id: ConceptId::Functions,
        title: "Functions",
        why_it_matters: "Functions let you name behavior once, reuse it everywhere, and keep programs organized.",
        explanation: "A function is a reusable block of logic. In Rust, functions are declared with `fn`, parameter types are explicit, and return types are written with `->`.\n\nWhy this helps beginners:\n- You can break a large problem into smaller named steps\n- Function signatures clearly document inputs and outputs\n- The compiler checks that callers pass the right types\n\nRust style tip: the last expression in a function is returned automatically if it has no semicolon. This keeps simple functions concise and readable.",
        code: r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}"#,
        code_explanation: "`a` and `b` are typed inputs. `-> i32` declares the output type. `a + b` is the returned value because it is the final expression without `;`.",
        beginner_mistake: "Mixing up expression returns and `return` statements, especially by adding a trailing semicolon and accidentally returning `()`.",
        recap: "Think of each function as a contract: typed inputs in, typed output out, with behavior isolated in one named unit.",
    }
}

fn concept_types() -> Concept {
    Concept {
        id: ConceptId::Types,
        title: "Types",
        why_it_matters: "Types are Rust's safety net: they describe what data is and what operations are valid.",
        explanation: "A type tells Rust what kind of value a variable holds and what you are allowed to do with it. Because Rust checks types at compile time, many bugs are caught before the app runs.\n\nCommon type groups:\n- Scalar types: `i32`, `f64`, `bool`, `char`\n- Compound types: tuples like `(i32, &str)` and arrays like `[i32; 3]`\n\nRust often infers types automatically, but adding explicit annotations is useful when learning and when you want code to be self-documenting.",
        code: r#"let x: i32 = 42;
let active: bool = true;
let pair: (i32, &str) = (7, "days");"#,
        code_explanation: "Each variable has a concrete compile-time type. Rust uses that type information to validate operations and prevent invalid combinations.",
        beginner_mistake: "Expecting automatic numeric conversions (for example using `i32` and `f64` together without casting).",
        recap: "When you understand the type, you understand the rules. Clear types make Rust code safer and easier to maintain.",
    }
}

fn concept_control_flow() -> Concept {
    Concept {
        id: ConceptId::ControlFlow,
        title: "if / match / loops",
        why_it_matters: "Control flow determines what your program does next, which branch runs, and how repeated work is performed safely.",
        explanation: "Control flow is how Rust programs make decisions and repeat tasks.\n\nThe core tools are:\n- `if` for simple true/false conditions\n- `match` for pattern-based branching (especially with enums)\n- loops (`loop`, `while`, `for`) for repetition\n\nA key Rust advantage is that `match` is exhaustive: you must handle every possible case. This catches missing logic during compilation rather than at runtime.",
        code: r#"let mood = "focused";
let message = match mood {
    "focused" => "Keep building",
    _ => "Take a break",
};"#,
        code_explanation: "`match` compares `mood` against each pattern. The `_` arm is a catch-all fallback, which makes the branch set complete.",
        beginner_mistake: "Using long chains of `if/else` where `match` would be clearer and safer, or forgetting to cover all match cases.",
        recap: "Use `if` for simple checks, `match` for structured branching, and loops for repetition. Prefer clarity and complete case handling.",
    }
}

fn concept_ownership() -> Concept {
    Concept {
        id: ConceptId::Ownership,
        title: "Ownership",
        why_it_matters: "Ownership prevents use-after-free and double-free bugs.",
        explanation: "Each value has one owner. Moving ownership transfers responsibility. When owner goes out of scope, value is dropped.",
        code: r#"let a = String::from("rust");
let b = a; // move
// println!("{a}"); // not allowed"#,
        code_explanation: "Strings live on the heap. Assigning `a` to `b` moves ownership.",
        beginner_mistake: "Expecting moved values to still be usable.",
        recap: "Ownership is the core rule that enables memory safety.",
    }
}

fn concept_borrowing_refs() -> Concept {
    Concept {
        id: ConceptId::BorrowingReferences,
        title: "Borrowing and References",
        why_it_matters: "Borrowing lets functions use data without taking ownership, which keeps code reusable and memory-safe.",
        explanation: "A reference is a borrowed view of a value. Borrowing means: \"I need to use this data, but I do not want to own it.\"\n\nRust has two main borrow forms:\n- `&T` (immutable reference): read-only access\n- `&mut T` (mutable reference): exclusive write access\n\nCore rule:\n- You can have many immutable references, OR one mutable reference\n- You cannot mix mutable and immutable borrows at the same time for the same value\n\nThis rule prevents data races and inconsistent reads while still letting you share data efficiently.",
        code: r#"fn len_of(s: &String) -> usize {
    s.len()
}"#,
        code_explanation: "The function receives `&String`, so it borrows the caller's string. It can read from `s`, but ownership stays with the caller.",
        beginner_mistake: "Assuming `&mut` is always better. Use immutable borrows by default and switch to mutable only when modification is required.",
        recap: "Borrowing is Rust's sharing model: read with `&T`, write with `&mut T`, and let the compiler enforce safe access timing.",
    }
}

fn concept_slices() -> Concept {
    Concept {
        id: ConceptId::Slices,
        title: "Slices",
        why_it_matters: "Slices let you pass and inspect parts of data efficiently without creating new owned values.",
        explanation: "A slice is a borrowed view into a contiguous range of data. It does not own the data; it points to part of data owned elsewhere.\n\nCommon slice forms:\n- `&str` for string slices\n- `&[T]` for slices of arrays/vectors\n\nWhy slices are useful:\n- no extra allocation for subranges\n- functions can accept flexible views (`&str` or `&[T]`) instead of owning full data\n- ownership remains clear while still enabling reuse",
        code: r#"let text = String::from("Rustacean");
let part: &str = &text[0..4]; // "Rust""#,
        code_explanation: "`part` borrows a range from `text`. It references bytes `0..4`, which is valid here because it aligns with UTF-8 character boundaries.",
        beginner_mistake: "Treating string indices as character indices. `String` is UTF-8 bytes, so slicing must respect valid character boundaries.",
        recap: "Slices are non-owning windows into existing data: fast, safe, and ideal for flexible function APIs.",
    }
}

fn concept_structs() -> Concept {
    Concept {
        id: ConceptId::Structs,
        title: "Structs",
        why_it_matters: "Structs let you model real entities in your program with named fields instead of loose, unrelated variables.",
        explanation: "A struct is a custom data type that groups related values under one name. Use structs when multiple pieces of data belong together conceptually (for example, a user profile, app settings, or a game character).\n\nWhy beginners should use structs early:\n- your data becomes self-documenting (`user.level` is clearer than separate variables)\n- code is easier to pass around as one unit\n- behavior can be attached with `impl` methods, keeping data and logic connected",
        code: r#"struct User {
    name: String,
    level: u32,
}"#,
        code_explanation: "`User` is a new type with named fields. Each field has an explicit type, and values are created with struct literal syntax like `User { name, level }`.",
        beginner_mistake: "Using tuples or many separate variables where a named struct would make the data model clearer.",
        recap: "Use structs to give shape to your data model. Named fields improve readability, maintainability, and API design.",
    }
}

fn concept_enums() -> Concept {
    Concept {
        id: ConceptId::Enums,
        title: "Enums",
        why_it_matters: "Enums model values that can be one of several known states, making state transitions explicit and safe.",
        explanation: "An enum defines a type with multiple possible variants. At runtime, a value is exactly one variant at a time.\n\nEnums are powerful because each variant can carry different data. This is ideal for modeling application states, message types, and success/error flows.\n\nWhy this helps beginners:\n- states are explicit in the type system\n- invalid \"impossible states\" are harder to represent\n- `match` works naturally with enums for complete branching",
        code: r#"enum AppState {
    Idle,
    Loading,
    Error(String),
}"#,
        code_explanation: "`Idle` and `Loading` carry no extra data, while `Error(String)` stores an error message. All three are part of one strongly typed state model.",
        beginner_mistake: "Representing state with plain strings like \"loading\"/\"error\" instead of enum variants, which loses compiler checks.",
        recap: "Use enums whenever a value can be in one of several named states. Pair them with `match` for safe, readable control flow.",
    }
}

fn concept_pattern_matching() -> Concept {
    Concept {
        id: ConceptId::PatternMatching,
        title: "Pattern Matching",
        why_it_matters: "Pattern matching gives you precise, readable branching while unpacking data safely.",
        explanation: "Pattern matching in Rust is more than an `if/else` replacement. It lets you branch based on data shape and value at the same time.\n\nWith `match`, you can:\n- check literals and ranges\n- destructure enums/tuples/options\n- bind inner values while branching\n- add guards (`if`) for extra conditions\n\nBecause `match` is exhaustive, the compiler ensures no case is accidentally ignored.",
        code: r#"let status = Some(3);
match status {
    Some(v) if v > 2 => println!("high"),
    Some(_) => println!("low"),
    None => println!("empty"),
}"#,
        code_explanation: "Line 1 wraps `3` in `Some`, so `status` has type `Option<i32>`. In the `match`, `Some(v) if v > 2` both destructures and checks a guard, so this arm prints `high` for values above 2. `Some(_)` means \"any other Some value\" (the underscore ignores the inner value). `None` handles the empty case, making the match exhaustive and compiler-approved.",
        beginner_mistake: "Writing broad `_` catch-alls too early, which can hide more specific patterns and reduce clarity.",
        recap: "Use pattern matching to branch and unpack data together. Prefer explicit arms first, then use `_` only as a final fallback.",
    }
}

fn concept_option_result() -> Concept {
    Concept {
        id: ConceptId::OptionResult,
        title: "Option and Result",
        why_it_matters: "Rust makes missing values and failures explicit in the type system, so error paths are visible and testable.",
        explanation: "`Option<T>` and `Result<T, E>` are core Rust enums for safe program flow.\n\n- `Option<T>` means: a value might exist (`Some`) or might be absent (`None`)\n- `Result<T, E>` means: an operation can succeed (`Ok`) or fail (`Err`)\n\nWhy this matters for beginners:\n- no hidden null behavior\n- no hidden exception flow\n- function signatures clearly communicate what callers must handle",
        code: r#"fn parse_port(raw: &str) -> Result<u16, std::num::ParseIntError> {
    raw.parse::<u16>()
}"#,
        code_explanation: "This function attempts to parse a `u16`. If parsing works, it returns `Ok(port)`; if parsing fails, it returns `Err(ParseIntError)`. The caller must decide how to handle both cases.",
        beginner_mistake: "Using `unwrap()` by default in application code, which can panic instead of handling `None`/`Err` gracefully.",
        recap: "Use `Option` for optional data and `Result` for fallible operations. Handle both branches intentionally instead of assuming success.",
    }
}

fn concept_vectors_strings() -> Concept {
    Concept {
        id: ConceptId::VectorsStrings,
        title: "Vectors and Strings",
        why_it_matters: "Vectors and strings are core owned collections used in almost every real Rust application.",
        explanation: "`Vec<T>` stores a growable list of values of one type. `String` stores owned UTF-8 text. Both are heap-allocated and follow Rust's ownership and borrowing rules.\n\nWhy beginners should learn these early:\n- they appear constantly in APIs and file/network work\n- they demonstrate real mutation (`push`, `push_str`) with `mut`\n- they help you practice owned vs borrowed data (`String` vs `&str`)\n\nThink of them as flexible containers you own and can grow over time.",
        code: r#"let mut nums = vec![1, 2, 3];
nums.push(4);
let mut text = String::from("Hello");
text.push_str(" Rust");"#,
        code_explanation: "`nums` grows from 3 items to 4 with `push`. `text` appends more characters with `push_str`. Both changes are allowed because the bindings are mutable (`mut`).",
        beginner_mistake: "Trying to modify a `String` or `Vec` without `mut`, or confusing owned `String` values with borrowed `&str` slices.",
        recap: "Use `Vec<T>` for dynamic collections and `String` for owned text. Reach for `&str` when you only need borrowed read access.",
    }
}

fn concept_error_handling() -> Concept {
    Concept {
        id: ConceptId::ErrorHandling,
        title: "Error Handling",
        why_it_matters: "Good error handling keeps programs reliable, debuggable, and predictable for users.",
        explanation: "Rust treats errors as values, usually with `Result<T, E>`. This makes failure paths explicit and forces code to choose how to respond.\n\nCommon strategy:\n- Return `Result` from functions that can fail\n- Use `?` to propagate errors upward cleanly\n- Handle errors at boundaries (CLI, HTTP handler, UI layer), where you can log, retry, or show user-friendly messages\n\nThis approach keeps core logic focused while still preserving detailed error information.",
        code: r#"use std::fs::read_to_string;

fn load_file(path: &str) -> Result<String, std::io::Error> {
    let data = read_to_string(path)?;
    Ok(data)
}"#,
        code_explanation: "`read_to_string(path)?` attempts the read. On success, execution continues with `data`. On failure, `?` returns early with the same error type, so callers can decide how to handle it.",
        beginner_mistake: "Using `panic!`/`unwrap()` for recoverable failures instead of returning and handling `Result`.",
        recap: "Model failure with `Result`, propagate with `?`, and handle near app boundaries where recovery decisions belong.",
    }
}

fn concept_modules() -> Concept {
    Concept {
        id: ConceptId::ModulesOrganization,
        title: "Modules: mod, pub, crate, use",
        why_it_matters: "Modules make larger Rust projects understandable and maintainable.",
        explanation: "`mod` declares modules, `pub` exposes items, `crate` refers to current crate root, and `use` brings paths into scope.",
        code: r#"mod ui;
pub mod models;

use crate::models::Concept;
"#,
        code_explanation: "This pattern appears in real codebases to separate concerns cleanly.",
        beginner_mistake: "Expecting items to be public without `pub`.",
        recap: "Good module boundaries are core to scaling Rust applications.",
    }
}

fn concept_traits_generics() -> Concept {
    Concept {
        id: ConceptId::TraitsGenerics,
        title: "Traits and Generics",
        why_it_matters: "Traits and generics let you write reusable code without sacrificing type safety or clarity.",
        explanation: "Traits describe shared behavior (what a type can do). Generics let one function or struct work with many concrete types.\n\nThink of it this way:\n- Trait = capability contract (behavior requirement)\n- Generic = placeholder type (`T`) filled in by concrete types later\n\nTogether they help you avoid copy-paste implementations while still keeping compile-time checks and strong IDE/compiler feedback.",
        code: r#"trait Summary {
    fn summary(&self) -> String;
}

fn show<T: Summary>(item: &T) {
    println!("{}", item.summary());
}"#,
        code_explanation: "`Summary` defines required behavior. `show<T: Summary>` means `show` accepts any type `T` that implements `Summary`, then safely calls `item.summary()`.",
        beginner_mistake: "Adding generics too early for simple code where concrete types would be easier to read and maintain.",
        recap: "Use traits to define behavior contracts and generics to reuse logic across types. Add them when repeated patterns become clear.",
    }
}

fn concept_lifetimes() -> Concept {
    Concept {
        id: ConceptId::LifetimesIntro,
        title: "Lifetimes (Intro)",
        why_it_matters: "Lifetimes let Rust prove that references never outlive the data they point to.",
        explanation: "A lifetime describes how long a reference is valid. Lifetimes are mostly inferred, but sometimes you need to write them explicitly to show how input and output references are related.\n\nImportant beginner mindset:\n- Lifetimes do not extend how long data lives\n- Lifetimes describe and verify relationships that already exist\n- They are a compile-time safety tool, not runtime behavior\n\nWhen the compiler cannot infer which input reference an output depends on, explicit lifetime parameters remove ambiguity.",
        code: r#"fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}"#,
        code_explanation: "`'a` ties both inputs and the output to the same lifetime parameter. The returned reference is guaranteed to be valid for at least the overlap allowed by the inputs.",
        beginner_mistake: "Trying to \"fix\" lifetime errors by adding random annotations instead of identifying the real ownership/borrow relationship.",
        recap: "Use lifetimes to describe reference relationships when inference is unclear. They help Rust prevent dangling references before runtime.",
    }
}

fn concept_cargo() -> Concept {
    Concept {
        id: ConceptId::CargoBasics,
        title: "Cargo Basics",
        why_it_matters: "Cargo is the standard Rust workflow tool for building, running, testing, and managing dependencies.",
        explanation: "Cargo is Rust's build system and package manager. It handles project scaffolding, dependency resolution, builds, tests, and reproducible workflows.\n\nBeginner workflow (most common loop):\n1. Edit code\n2. Run `cargo check` for fast feedback\n3. Run `cargo run` to execute\n4. Run `cargo test` to validate behavior\n\nCargo reads `Cargo.toml` for project metadata and dependencies, and uses `Cargo.lock` to keep dependency versions consistent across machines.",
        code: r#"cargo new my_app
cargo run
cargo check
cargo test"#,
        code_explanation: "What each command does:\n\n`cargo new my_app`\nCreates a new Rust project folder with starter files like `Cargo.toml` and `src/main.rs`.\n\n`cargo run`\nBuilds your project (and dependencies if needed), then runs the resulting binary.\n\n`cargo check`\nPerforms fast compile-time checking (types, borrows, lifetimes) without producing a final runnable binary.\n\n`cargo test`\nBuilds test targets and runs all discovered `#[test]` functions, then reports pass/fail results.",
        beginner_mistake: "Treating Cargo as just a run command instead of the full development workflow (check, build, test, dependency management).",
        recap: "Use Cargo as your daily Rust control center: create, check, run, test, and manage dependencies from one consistent toolchain.",
    }
}

cargo run
#    Compiling minigrep v0.1.0 (D:\Programming\Rust-Learning\12-An-IO-Project\minigrep)
#    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
#     Running `target\debug\minigrep.exe`
#[src\main.rs:5:5] args = [
#    "target\\debug\\minigrep.exe",
#]

cargo run -- needle haystack
#     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
#     Running `target\debug\minigrep.exe needle haystack`
#[src\main.rs:5:5] args = [
#    "target\\debug\\minigrep.exe",
#    "needle",
#    "haystack",
#]

cargo run -- needle haystack
# Searching for needle
# In file haystack

cargo run -- test sample.txt
# Searching for test
# In file sample.txt



$Env:IGNORE_CASE=1; cargo run -- to poem.txt
#Are you nobody, too?
#How dreary to be somebody!
#To tell your name the livelong day
#To an admiring bog!

# Remove Ignore-Case env var:
Remove-Item Env:IGNORE_CASE

# Test after deleting:
cargo run -- to poem.txt
#Are you nobody, too?
#How dreary to be somebody!
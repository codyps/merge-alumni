
1. download csv for both the "ChurchWindows" and "OnRealm" tables
2. [install rust](https://rustup.rs/)
3. run:

    RUST_BACKTRACE=1 cargo run -- -c Alumni\ File.xlsx\ -\ ChurchWindows.csv -r Alumni\ File.xlsx\ -\ OnRealm.csv -o out_no_a_or_b.csv -f '^(A|B).*'`
    RUST_BACKTRACE=1 cargo run -- -c Alumni\ File.xlsx\ -\ ChurchWindows.csv -r Alumni\ File.xlsx\ -\ OnRealm.csv -o out_all.csv 

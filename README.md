```sh
# this is a fairly aggressive linter
cargo clippy -- -W clippy::nursery -W clippy::pedantic -W clippy::unwrap_used -W clippy::expect_used -A clippy::future_not_send -A clippy::must_use_candidate -A clippy::missing_errors_doc -A clippy::unused_async
```

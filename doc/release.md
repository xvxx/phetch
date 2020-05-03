# phetch release process

- update CHANGELOG.md
- $ cargo test
- $ cargo release --dry-run --prev-tag-name=v1.x.x patch
- <edit https://github.com/xvxx/phetch/releases>
- $ cd ../[homebrew-code]
- $ make VERSION=v1.x.x


[homebrew-code]: https://github.com/xvxx/homebrew-code

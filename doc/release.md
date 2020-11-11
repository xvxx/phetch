# phetch release process

- update CHANGELOG.md
- $ make test
- $ cargo release --prev-tag-name=v1.x.x -n
- $ cargo release --prev-tag-name=v1.x.x
- edit https://github.com/xvxx/phetch/releases
- $ cd ../homebrew-code
- $ make VERSION=v1.x.x


[homebrew-code]: https://github.com/xvxx/homebrew-code

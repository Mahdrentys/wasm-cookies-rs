# Contributing

This repository follows the [Github Flow](https://githubflow.github.io): feature and bug fix branches are based off master and are merged to master. This is allows very fast evolution. Indeed, there is no pre-releases, and every feature merged makes a new release.

## Continuous integration

Each time a branch other than `master` is pushed, the following happens in Github Actions:

- The source code formatting is checked.
- The crate is built.
- The tests are run.

Each time the `master` branch is pushed, the following happens in Github Actions:

- The source code formatting is checked.
- The crate is built.
- The tests are run.
- If the version number was updated, a Github Release is created with tag according to the version number.
- If the version number was updated, a new version of the crate is published to crates.io.

## Commit messages

Commit messages must be written in imperative tense, with a first capital letter, and without dot at the end.

## Pull requests

Branch names must be written in lowercase, with dash separated words. For a bug fix branch, the branch name must prefixed by "fix/".

Before submitting your pull request, you must update the version number in `Cargo.toml`, in a commit whose message is "Update version number". The version number follows semantic versionning: if it's a new feature, you must increment the minor number and reset the patch number to 0, and if it's a bug fix, you must increment the patch number. If your pull request only makes changes on things that are not directly related to the crate (for example if you edit the Github Actions workflows), don't update the version number.

If the master branch is ahead of your branch, you must merge it into your branch to resolve conflicts. You might have to update the version number again if it has been upgraded since you did it.

If your pull request fixes a bug that is mentionned in an issue, don't forget to add "Closes #123" (where 123 is the issue number) in your pull request description.

## Releases

Each time a pull request is merged, a new Github Release is created by the CI system, and the contributor who merged is responsible for editing the release notes (in markdown) in the format:

```markdown
### Features
- Feature 1 without dot at the end
- Feature 2 without dot at the end

### Bug fixes
- Bug fix 1 without dot at the end
- Bug fix 2 without dot at the end

See #123.
```

Where `#123` is the ID of the corresponding pull request.
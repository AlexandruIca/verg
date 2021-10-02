# Foreword
I'm not writing this expecting to have people contributing to this project(of course that would be awesome), I doubt this project will even be _seen_ by other people, I'm mostly writing this for myself.

# Before you start working on something
Create a new issue if it doesn't already exist. Give it an appropiate title and describe in as much detail as you can what the issue is.

After you have an issue to work on you should create a new branch. Never commit directly to `main`, that's only meant to be merged _into_. The name of the branch should follow this scheme:
```
VG-issue_number-Short-description-of-what's-being-worked-on
```
This is so that it's easy to keep track of changes being made to the project.

# Create tests
This project isn't test-driven, but whenever you add/change something you should accompany that with some tests so that it's clear what the intent of the addition/modification is.

# Commit structure
```
component: Short description, 80-100 columns (#issue_number)

Optionally more text
```
The first line of the commit message should start with the component that you're working on, that could be one of:
- `misc`: anything that's not directly related to the project itself, for example: editing `.gitignore`, solving merge conflicts
- `build`: anything related to the build process, for example: editing `shell.nix`, editing `Cargo.toml`
- `core`: anything related to the inner workings of this lib
- `test`: anything code-related that you modify in `tests/`
- `ci`: anything related to `.github/workflows/`
- `docs`: anything documentation-related, for example: editing `.md` files, writing rust docs

For example, if I want to commit a modification to `README.md` a commit message could look like this:
```
docs: Provide some info in the README (#42)

Here I could write in more detail about the change
```
If you feel like a commit may fit inside two components you can concatenate them. For example, let's say I modify the CI workflow and change a script that generates documentation. The commit message may look like:
```
ci:docs: <What changed> (#13)
```
You shouldn't need more than two components, if you do then maybe that commit should have been split into multiple commits. Keep commits simple and to the point. You can take a look [here](https://chris.beams.io/posts/git-commit/) for some additional advice on how to write commit messages.

# After you're done working
You should run tests, run clippy and run rustfmt to avoid failed CI builds. If everything's ok and the CI builds successfully you can create a PR to be merged into main. PRs might not be needed because I'm by myself but it's still nice to have an organized way to see project history.
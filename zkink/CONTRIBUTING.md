# Contributing to ink!

First of all, thank you for taking your time to contribute to ink!

> [I don't want to contribute, I just have some questions! :S](#I-dont-want-to-contribute-I-just-have-some-questions)

## Table of Contents
* [Code of Conduct](#code-of-conduct)
* [During Development](#during-development)
    * [Commits](#commits)
    * [Checklist](#checklist)
* [Continuous Integraton](#continuous-integration)
* [Issues and Pull Requests](#issues-and-pull-requests)
    * [Issues](#issues)
    * [Pull Requests](#pull-requests)
* [Questions](I-dont-want-to-contribute-I-just-have-some-questions)

## Code of Conduct

We are a friendly and welcoming open source community.
You can find our code of conduct [here](CODE_OF_CONDUCT.md).

## During Development

### Commits

Don't be afraid to have a bunch of commits while working on a pull-request. We end up
squashing all of them before merging to the `master` branch anyways.

But please keep your commits small and descriptive. A good rule of thumb is to 
bundle semantic changes together in one commit. It makes the review
process easier - which means you get a 🟩 from Github sooner (that's why you're
contributing in the first place anyways, right?)

To help you out here's a [really good post](https://cbea.ms/git-commit/) on how to write good commit
messages.

### Checklist

Below is a checklist for you before doing a pull request.

Following these will ensure that your pull request is going to be accepted.

1. Run `rustfmt` automatically.
1. Run `clippy` on your changes.
1. Run tests via `cargo test --release` for off-chain testing.
1. For critical parts perform some manual on-chain tests.
1. Build the code and run tests also for the `wasm32` target.
1. Try to run some examples and see if they are still working correctly.
1. Sometimes clippy lints can be unfortunate or even buggy and it can be very hard to fix those.
  In these situations you may skip the clippy lint with `#[clippy::skip]`, however,
  note that this always requires a good rational as a side-comment and probably link to the eventual clippy bug.
1. **Important** Strive for simple, clean and concise code.
  If your code is very complex - because it is trying to accomplish complex things - try to think about how another aproach or design could simplify it.
1. We won't accept a pull request with `FIXME` or `TODO` comments in it.
   Please try to fix them by yourself and resolve all remaining to-do items.
   If that is not possible then write an issue for each of them and link to the source line and commit with a proper description. For more information go [here](#Issues-&-pull-requests).
1. Document everything properly that you have written, refactored or touched. Especially keeping documentation up-to-date is very important. For larger portions please try to also update the ink! wiki or write a new entry there.
1. Write tests for your code. If your code is hard to test, try to find a design that allows for testing.
1. If needed also update the [`README`](README.md).
1. For performance critical parts we also want additional performance tests.
1. If you implement a fix for a reported bug also include a regression test alongside the fix if possible.
1. When commenting or documenting code make use of proper punctuation.
   This might seem pedantic but we believe that in essence this is going to improve overall comment and documentation quality.
1. If possible try to sign your commits, e.g. using GPG keys. For more information about this go [here](https://help.github.com/en/articles/signing-commits).

## Continuous Integration

Our [continuous integration (CI)](https://github.com/paritytech/ink/blob/master/.gitlab-ci.yml) will check for the following properties of all changes.

1. Is `rustfmt` happy with it ?
    - `cargo fmt --all`
1. Is `clippy` happy with it?
    - `cargo clippy --all-targets --all-features`
1. Does the code still compile?
    - `cargo check --all-features`
1. Do all the examples still compile?
    - `cargo +nightly contract check --manifest-path ./examples/.../Cargo.toml`
1. Is the `wasm-32` target still compiling?
    - `cargo check --no-default-features --target wasm32-unknown-unknown`
1. Are all the tests passing?
    - `cargo test --all-features --workspace`
1. Are all the tests for the examples passing?
    - `cargo +nightly test --manifest-path ./examples/.../Cargo.toml`
1. Is the test code coverage increasing or at least stable?
1. Has the size of the example contract binaries changed?

Only if our very demanding CI is happy with your pull request we will eventually merge it.
Exceptions confirm the rule!

## Issues and Pull Requests

Please always respect our [code of conduct](CODE_OF_CONDUCT.md) when writing issues and pull requests or taking part in any kind of discussion.

### Issues

You want to write an issue if you have a
- feature request
- bug report
- technical question

about the ink! project.

> Please use GitHub's search functionality to see if the question has already been asked,
the feature has already been proposed or the bug has already been reported.

Make use of the accompanied issue template.

### Pull Requests

Before working on a pull request please make sure that the work has not already been made, e.g. by another pull request solving the same thing.

For complex implementations you are advised to first discuss the feature implementation or bug fix using an issue.

A pull request should be doing or implementing exactly one semantic issue. So for example, when you refactor the code base in a pull request you should not also include code formattings into the same pull request.

It's totally fine to extract changes made in one pull request to multiple pull requests. It makes the review process easier (and hey, more 🟩 for you!).

If you've already opened a pull request, avoid force-pushing any new changes.

For a nice list of hints visit this [link][GitHub Perfect Pull Reqest].

## I don't want to contribute, I just have some questions

For questions about the ink! project, about Parity Technologies or general technical
related questions you are welcome to contact us via [Element][Riot-Smart-Contracts-ink]. For
technical questions specifically about the ink! and its sub-projects you may also file an issue.
For more information about filing issues go [here](#issues-and-pull-requests).

[Riot-Smart-Contracts-ink]: https://riot.im/app/#/room/#ink:matrix.parity.io

[GitHub Perfect Pull Reqest]: https://github.blog/2015-01-21-how-to-write-the-perfect-pull-request/

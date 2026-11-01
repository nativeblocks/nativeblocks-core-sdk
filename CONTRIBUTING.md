Contributing
============

Our goal with this project is to build a simple, fast, and reliable runtime that lets developers
build server-driven UI for their Android and iOS apps. We have scoped the project deliberately to
cover common use cases well, and made it extensible through custom blocks and actions so it can
handle exotic use cases too.

Keeping the project small and stable limits our ability to accept new contributors. We are not
seeking new committers at this time, but some small contributions are welcome.

If you've found a bug, please open an issue with a minimal reproduction, ideally a failing test
case, so we can study and fix it.

If you have a new feature idea, please build it outside this repository first. Most features fit as
custom blocks, actions, or a library on top of the public API. If you build something that
integrates with Nativeblocks, tell us so that we can link it!

Using AI
--------

We use AI tools in our own work on Nativeblocks, and you're welcome to use them too. For us, they
help with the thinking, and the product is still built by people who know every change and why it's
there. We ask the same of your contributions.

### ✅ Welcome

* Using AI to explore the code and ask questions about it.
* Brainstorming ideas that you then work out and build yourself.
* Getting a second opinion on code or writing you did yourself.

### ❌ Not welcome

* Pull requests created by AI.
* Commits made by AI.
* Code written by AI.
* Issues and comments written by AI.
* AI slop of any kind. If you can't explain what you're submitting and why, please don't submit it.

Please don't guess how a contribution was made. Writing style isn't evidence, and people writing in
a second language are often the ones wrongly suspected.

Code Contributions
------------------

Get working code on a personal branch with tests passing before you submit a PR. A change to the
Rust core must pass on all platforms, because all host SDKs ship the same core.

Please make every effort to follow existing conventions and style in order to keep the code as
readable as possible. Keep diffs small and focused on one change.

Contribute code changes through GitHub by forking the repository and sending a pull request.

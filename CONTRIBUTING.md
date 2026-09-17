Contributing
============

Our goal with this project is to build a simple, fast, and reliable runtime that lets developers
drive their Android and iOS apps remotely, over the air. We have scoped the project deliberately to cover common use
cases well, and made it
extensible through custom blocks and actions so it can handle exotic use cases too.

Keeping the project small and stable limits our ability to accept new contributors. We are not
seeking new committers at this time, but some small contributions are welcome.

If you've found a bug, please open an issue with a minimal reproduction, ideally a failing test
case, so we can study and fix it.

If you have a new feature idea, please build it outside this repository first. Most features fit as
custom blocks, actions, or a library on top of the public API. If you build something that
integrates with Nativeblocks, tell us so that we can link it!

LLM Usage Policy
----------------

Using LLMs while working on Nativeblocks is conditionally allowed, when done with care. LLMs are not
a substitute for thought, and we don't allow them to be used in ways that risk losing our shared
understanding of the code.

### ✅ Allowed

Any use where you are the only one who sees the output. For example:

* Asking an LLM questions about the codebase privately.
* Asking an LLM to summarize an issue or PR for yourself. Don't repost the summary.
* Asking an LLM to privately review your code or prose.

### ❌ Banned

* Code originally created by an LLM, no matter how small the change. This includes typo fixes,
  links, and other trivial edits.
* Using an LLM to find bugs, then reporting or fixing them.
* Issues, PR descriptions, and comments originally created by an LLM.
* Documentation originally created by an LLM.
* Processes that need an LLM to follow them. Documentation is written for humans first.
* Treating an LLM review as enough to merge or reject a change. LLM reviews are advisory only, and
  they never replace your own self-review.

### ⚠️ Allowed with disclosure

Say in the PR or issue that you used an LLM. New contributors should expect more scrutiny, because
they haven't built trust with the reviewers yet.

* **Machine translation.** Allowed but discouraged. Posting your original message next to the
  translation is always fine.

### Be Honest

Be honest about whether and how much you used an LLM. If you're not sure where your use falls, ask
a maintainer before you open the PR. Deliberately misrepresenting your LLM use gets a warning first,
then a ban.

Your contribution is your responsibility. You can't blame an LLM for it.

"Originally created by an LLM" means text or code an LLM generated, even if a human edited it
afterwards. Editing doesn't change where it came from. Autocomplete counts the same as a chat window,
though its output is usually trivial.

### Not Your Job to Play Detective

Don't police whether someone used an LLM. Writing style is not evidence: people writing in a second
language, neurodivergent people, and people who over-explain are the most likely to be wrongly
accused. If someone clearly broke the rules, point them to this policy. Otherwise, tell a maintainer
privately instead of accusing them in public.

Never harass a contributor for using an LLM.

Code Contributions
------------------

Get working code on a personal branch with tests passing before you submit a PR. A change to the
Rust core must pass on all platforms, because all host SDKs ship the same core.

Please make every effort to follow existing conventions and style in order to keep the code as
readable as possible. Keep diffs small and focused on one change.

Contribute code changes through GitHub by forking the repository and sending a pull request.

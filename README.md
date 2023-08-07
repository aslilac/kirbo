<h1 align="center">
<img alt="Kirbo" src="https://cdn.mckayla.cloud/-/9dd72ce86ff44066bd0f2a6b902373ad/Logo.webp" height=120 />
</h1>

A secure and fast npm/yarn alternative

### Coming from Yarn Classic?

Migrating is easy. All you need to do is install Kirbo, and run

```sh
kirbo
```

We'll migrate your yarn.lock to a kirbo.lock, and you'll be ready to go.

### Workspaces

Kirbo has predictable, reliable workspaces support, to enable you to get things done. No need to add another configuration file, or change your existing config. Kirbo uses the same workspace configuration as npm and yarn.

### Secure development

Install dependencies confidently, knowing that they won't be able to execute arbitrary code during installation. You'll also get version locking and immutability, like you'd expect from any modern package manager.

### Faster feedback in CI

No need to install dependencies first. Why wait for a full install just to run your code formatter? Kirbo is smart enough to install just the necessary bits as needed.

```yaml
- run: kirbo -- prettier --check . # will *only* install prettier, and then run it
- run: kirbo lint # will look up your `lint` script, and install necessary packages
- run: kirbo # install everything to prepare for building
- run: kirbo build # will run your `build` script from your package.json
```

### Offline support

Kirbo usually won't need a network connection to install your dependencies. It'll always try to find what it needs locally first, only reaching out to the network if necessary. No telemetry. No bullshit.

You can even use the `--offline` flag to fail if the network is needed.

### Drop-in npm and yarn replacement

Kirbo can directly import your existing yarn.lock file, and we'll check our resolution algorithm against your existing node_modules/ directory to make sure that everything resolves the way you expect. If things don't align, you'll get a warning up front, rather than surprise bugs down the road.

Most of the commands you're used to will still just work. Commands like `kirbo install`, `kirbo add -D typescript`, `kirbo fmt`, `kirbo run test` all do what you probably expect.

### Insight into your dependencies

Get warnings about dependencies that are slowing down your installations. Large dependencies aren't the only risk, small dependencies, or dependencies that bring in large transitive trees can have a big impact too. See what dependencies are getting reused effectively and which ones require duplication. See what action you can take to make things faster.

`kirbo insight`

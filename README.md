A template for web applications.

To use the template,

1. Make a copy/clone of the repo
2. Delete the `.git` directory
3. Replace the string `web_template` everywhere in the repo, including file names.
4. Initialize git, e.g. `git init`

Future improvement idea: use [cargo-generate](https://github.com/cargo-generate/cargo-generate).

## Tips

Configure log levels for different packages and modules, e.g.

```shell
RUST_LOG=webapp_template_web=debug,webapp_template_app=debug,tower_http=debug,axum::rejection=trace
```
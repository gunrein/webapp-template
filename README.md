A template for web applications.

## Tips

Configure log levels for different packages and modules, e.g.

```shell
RUST_LOG=webapp_template_web=debug,webapp_template_app=debug,tower_http=debug,axum::rejection=trace
```
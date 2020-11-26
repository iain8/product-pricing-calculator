# Wholesale Pricing Calculator

A simple calculator for pricing your goods. Uses seed and Tailwind CSS.

## 5. Prepare your project for deploy

1. Run `cargo make verify` in your terminal to format and lint the code.
1. Run `cargo make build_release`.
1. Upload `index.html` and `pkg` into your server's public folder.
   - Don't forget to upload also configuration files for your hosting, see the [Netlify](https://www.netlify.com/) one below.

```toml
# netlify.toml
[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

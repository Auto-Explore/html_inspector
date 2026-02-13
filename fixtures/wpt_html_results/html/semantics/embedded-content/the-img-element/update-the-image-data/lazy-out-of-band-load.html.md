# html/semantics/embedded-content/the-img-element/update-the-image-data/lazy-out-of-band-load.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/update-the-image-data/lazy-out-of-band-load.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Lazy loaded image doesn't get loaded eagerly after another image gets loaded with the same source</title>
<link rel="help" href="https://github.com/whatwg/html/issues/10671">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
async function run_test(prop, image, t) {
  let value = `${image}?lazy-out-of-band-load-` + Math.random();
  let img = document.createElement("img")
  img[prop] = value;
  img.loading = "lazy";
  img.style.display = "none";
  img.addEventListener("error", t.unreached_func("error: should never try to load"));
  img.addEventListener("load", t.unreached_func("load: should never try to load"));
  document.body.appendChild(img);

  await new Promise(r => t.step_timeout(r, 100));

  // now load another image with the same src, but not lazy.
  img = document.createElement("img");
  img[prop] = value;

  await new Promise(r => {
    img.addEventListener("load", r, { once: true });
    img.addEventListener("error", r, { once: true });
    document.body.appendChild(img);
  });

  // Wait a bit to make sure we don't get a broken load.
  await new Promise(r => t.step_timeout(r, 100));
}

for (let prop of ["src", "srcset"]) {
  for (let path of ["/images/green.png", "/images/not-found"]) {
    promise_test(t => run_test(prop, path, t), `${prop} = ${path}`);
  }
}
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/update-the-image-data/lazy-out-of-band-load.html"
}
```

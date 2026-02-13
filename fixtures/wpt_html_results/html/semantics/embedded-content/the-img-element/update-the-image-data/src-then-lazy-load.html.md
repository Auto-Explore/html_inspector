# html/semantics/embedded-content/the-img-element/update-the-image-data/src-then-lazy-load.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/update-the-image-data/src-then-lazy-load.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Image shouldn't load synchronously out of the document.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
async function run_test(prop, image, repetitions, t) {
  for (let i = 0; i < repetitions; ++i) {
    let img = document.createElement("img")
    img[prop] = `${image}?src-then-lazy-load-` + Math.random();
    img.loading = "lazy";
    img.style.display = "none";
    img.addEventListener("error", t.unreached_func("error: should never try to load"));
    img.addEventListener("load", t.unreached_func("load: should never try to load"));
    document.body.appendChild(img);
    await new Promise(r => t.step_timeout(r, 500));
  }
}

for (let prop of ["src", "srcset"]) {
  for (let path of ["/images/green.png", "/images/not-found"]) {
    for (let repetitions of [1, 2]) {
      promise_test(t => run_test(prop, path, repetitions, t), `${prop} = ${path} ${repetitions} time(s)`);
    }
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
  "source_name": "html/semantics/embedded-content/the-img-element/update-the-image-data/src-then-lazy-load.html"
}
```

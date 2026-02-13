# html/semantics/embedded-content/the-img-element/disconnected-image-loading-lazy.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/disconnected-image-loading-lazy.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
async_test(t => {
  x = new Image();
  x.loading = "auto";
  x.src = "resources/image.png?auto";
  x.onload = t.step_func_done();
  t.step_timeout(t.unreached_func("Disconnected loading=auto image loads " +
                                  "successfully, and doesn't timeout"), 2000);
}, "loading=auto for disconnected image");

async_test(t => {
  x = new Image();
  x.loading = "eager";
  x.src = "resources/image.png?eager";
  x.onload = t.step_func_done();
  t.step_timeout(t.unreached_func("Disconnected loading=eager image loads " +
                                  "successfully, and doesn't timeout"), 2000);
}, "loading=eager for disconnected image");

async_test(t => {
  x = new Image();
  x.loading = "lazy";
  x.src = "resources/image.png?lazy";
  x.onload = t.unreached_func("Disconnected loading=lazy image loads lazily.");
  t.step_timeout(t.step_func_done(), 2000);
}, "loading=lazy for disconnected image");
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/disconnected-image-loading-lazy.html"
}
```

# html/semantics/embedded-content/the-img-element/empty-src-no-current-request.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/empty-src-no-current-request.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>src = "" doesn't trigger a sync load if there's no existing current request</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1905646">
<script>
  promise_test(async function(t) {
    let img = new Image();
    img.src = "";
    img.loading = "lazy";
    img.onload = t.unreached_func("should not trigger a load event");
    img.onerror = t.unreached_func("should not trigger an error event");
    // Shouldn't fire any event since it's in the lazy state.
    await new Promise(r => t.step_timeout(r, 0));
    await new Promise(r => t.step_timeout(r, 0));

    // We're about to append it to the document, which should trigger the (lazy) load (and in this case error event).
    let error = new Promise(r => {
      img.onerror = r;
    });
    document.documentElement.appendChild(img);
    await error;
  }, "Without srcset");

  promise_test(async function(t) {
    let img = new Image();
    img.src = "";
    img.srcset = "/images/green.png";
    img.loading = "lazy";
    img.onload = t.unreached_func("should not trigger a load event");
    img.onerror = t.unreached_func("should not trigger an error event");
    // Shouldn't fire any event since it's in the lazy state.
    await new Promise(r => t.step_timeout(r, 0));
    await new Promise(r => t.step_timeout(r, 0));

    // We're about to append it to the document, which should trigger the (lazy) load.
    let load = new Promise(r => {
      img.onload = r;
    });

    document.documentElement.appendChild(img);
    await load;
  }, "With srcset");
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
  "source_name": "html/semantics/embedded-content/the-img-element/empty-src-no-current-request.html"
}
```

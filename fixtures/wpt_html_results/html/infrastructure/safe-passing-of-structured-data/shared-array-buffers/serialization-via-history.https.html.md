# html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/serialization-via-history.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/serialization-via-history.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>SharedArrayBuffers cloning via history's methods invoking StructuredSerializeForStorage</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#structuredserializeforstorage">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-history-pushstate">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-history-replacestate">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
"use strict";

for (const method of ["pushState", "replaceState"]) {
  test(() => {
    assert_throws_dom("DataCloneError", () => {
      history[method](new SharedArrayBuffer(), "dummy title");
    });
  }, `history.${method}(): simple case`);

  test(() => {
    let getter1Called = false;
    let getter2Called = false;
    assert_throws_dom("DataCloneError", () => {
      history[method]([
        { get x() { getter1Called = true; return 5; } },
        new SharedArrayBuffer(),
        { get x() { getter2Called = true; return 5; } }
      ], "dummy title");
    });

    assert_true(getter1Called, "The getter before the SAB must have been called");
    assert_false(getter2Called, "The getter after the SAB must not have been called");
  }, `history.${method}(): is interleaved correctly`);
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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/serialization-via-history.https.html"
}
```

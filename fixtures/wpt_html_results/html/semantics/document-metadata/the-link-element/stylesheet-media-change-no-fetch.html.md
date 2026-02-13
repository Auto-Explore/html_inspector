# html/semantics/document-metadata/the-link-element/stylesheet-media-change-no-fetch.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/stylesheet-media-change-no-fetch.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>stylesheet should not re-fetch when media changes. The media change should apply immediately</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/links.html#link-type-stylesheet">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#processing-the-media-attribute">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  p {
    color: red;
  }
</style>
<body></body>
<script>
  promise_test(async t => {
    const link = document.createElement("link");
    function waitForStylesheet() {
      return new Promise(resolve => link.addEventListener("load", resolve, {once: true}));
    }
    link.media = "none";
    link.rel = "stylesheet";
    link.href = "resources/good.css";
    const p = document.createElement("p");
    p.textContent = "hello";
    document.body.appendChild(p);
    document.body.appendChild(link);
    await waitForStylesheet();
    assert_equals(getComputedStyle(p).color, "rgb(255, 0, 0)");
    link.media = "all";
    assert_equals(getComputedStyle(p).color, "rgb(0, 128, 0)");
    const loaded = waitForStylesheet().then(() => "load");
    const timeout = new Promise(resolve => t.step_timeout(() => resolve("timeout"), 100));
    const result = await Promise.race([loaded, timeout]);
    assert_equals(result, "timeout");
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 524,
        "byte_start": 516,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1404,
        "byte_start": 524,
        "col": 9,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1413,
        "byte_start": 1404,
        "col": 1,
        "line": 36
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-link-element/stylesheet-media-change-no-fetch.html"
}
```

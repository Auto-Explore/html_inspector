# html/semantics/text-level-semantics/the-a-element/a-type.historical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-a-element/a-type.historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>The type attribute is purely advisory</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/links.html#attr-hyperlink-type">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>

<body>
<iframe name="i"></iframe>
<a id="link" href="resources/plain-text.unknown" target="i" type="text/html">click me</a>
<script>
async_test(t => {
  let link = document.getElementById("link");
  link.click();

  let iframe = document.querySelector("iframe");
  iframe.onload = t.step_func_done(() => {
    assert_true(iframe.contentWindow.location.href.endsWith(".unknown"));
    assert_equals(iframe.contentDocument.contentType, "text/plain");
  });
}, "type attribute on anchor doesn't cause document to be loaded as HTML");
</script>
</body>
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
  "source_name": "html/semantics/text-level-semantics/the-a-element/a-type.historical.html"
}
```

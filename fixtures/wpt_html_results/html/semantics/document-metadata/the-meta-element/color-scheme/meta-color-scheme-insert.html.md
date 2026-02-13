# html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-insert.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-insert.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Insert color-scheme meta tags</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#meta-color-scheme">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/compute-root-color-scheme.js"></script>
<!--
  NOTE: This test assumes that the browser's default color-scheme is "light",
  see https://github.com/web-platform-tests/wpt/pull/31268 for reasoning
-->
<script>
  function createMeta(content) {
    const meta = document.createElement("meta");
    meta.setAttribute("name", "color-scheme");
    meta.setAttribute("content", content);
    return meta;
  }

  assert_root_color_scheme("light", "Initial color-scheme");

  document.head.appendChild(createMeta("dark"));
  assert_root_color_scheme("dark", "Inserted meta color-scheme applies");

  document.head.insertBefore(createMeta("light"), document.head.lastChild);
  assert_root_color_scheme("light", "Inserted meta color-scheme before existing in head applies");
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
  "source_name": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-insert.html"
}
```

# html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-single-value-in-shadow-tree.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-single-value-in-shadow-tree.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Meta color-scheme in shadow-tree should not apply</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#meta-color-scheme">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/compute-root-color-scheme.js"></script>
<!--
  NOTE: This test assumes that the browser's default color-scheme is "light",
  see https://github.com/web-platform-tests/wpt/pull/31268 for reasoning
-->
<script>
  const host = document.createElement("div");
  host.id = "host";
  document.head.appendChild(host);
  const root = host.attachShadow({mode:"open"});
  const meta = document.createElement("meta");
  meta.setAttribute("name", "color-scheme");
  meta.setAttribute("content", "dark");
  root.appendChild(meta);

  assert_root_color_scheme("light", "Meta color-scheme in shadow tree does not apply.");
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
  "source_name": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-single-value-in-shadow-tree.html"
}
```

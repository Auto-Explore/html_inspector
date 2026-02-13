# html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-attribute-changes.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-attribute-changes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Meta color-scheme - attribute changes</title>
<meta id="meta" name="color-scheme" content="dark">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#meta-color-scheme">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/compute-root-color-scheme.js"></script>
<!--
  NOTE: This test assumes that the browser's default color-scheme is "light",
  see https://github.com/web-platform-tests/wpt/pull/31268 for reasoning
-->
<script>
  assert_root_color_scheme("dark", "Meta color-scheme initially 'dark'.");

  meta.removeAttribute("name");
  assert_root_color_scheme("light", "Removed name attribute from meta color-scheme.");

  meta.setAttribute("name", "color-scheme");
  assert_root_color_scheme("dark", "Set meta name to color-scheme.");

  meta.setAttribute("content", "");
  assert_root_color_scheme("light", "Set content attribute of meta color-scheme to empty string.");

  meta.setAttribute("content", ",,invalid");
  assert_root_color_scheme("light", "Set content attribute of meta color-scheme to an invalid value.");

  meta.setAttribute("content", "light");
  assert_root_color_scheme("light", "Set content attribute of meta color-scheme to 'light'.");

  meta.setAttribute("content", "dark");
  assert_root_color_scheme("dark", "Set content attribute of meta color-scheme to 'dark'.");

  meta.removeAttribute("content");
  assert_root_color_scheme("light", "Removed the content attribute of meta color-scheme.");
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
  "source_name": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-attribute-changes.html"
}
```

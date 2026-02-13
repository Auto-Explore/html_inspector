# html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-first-valid-applies.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-first-valid-applies.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Multiple color-scheme meta tags - first valid applies</title>
<meta name="color-scheme">
<meta name="color-scheme" content>
<meta name="color-scheme" content="">
<meta name="color-scheme" content="light,dark">
<!-- This is first with a valid content value -->
<meta name="color-scheme" content="dark">
<meta name="color-scheme" content="light">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#meta-color-scheme">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/compute-root-color-scheme.js"></script>
<script>
  assert_root_color_scheme("dark", "Tree order decides which meta color-scheme applies.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 111,
        "byte_start": 85,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-first-valid-applies.html"
}
```

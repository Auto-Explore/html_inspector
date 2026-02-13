# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_block_modals-3.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_block_modals-3.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>iframe sandbox without allow_modals (prompt)</title>
<link rel="author" title="Igalia" href="https://www.igalia.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-iframe-sandbox">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-iframe-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe sandbox="allow-scripts"></iframe>
<script src="support/iframe_sandbox_block_modals.js"></script>
<script>
    runTest("prompt", null);
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_block_modals-3.html"
}
```

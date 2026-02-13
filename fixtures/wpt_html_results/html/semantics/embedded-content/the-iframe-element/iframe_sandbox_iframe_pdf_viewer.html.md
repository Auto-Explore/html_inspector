# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_iframe_pdf_viewer.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_iframe_pdf_viewer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<meta charset="utf-8">
<title>Sandbox: Block plugins inside iframe with sandbox attribute</title>

<!--
  https://github.com/whatwg/html/issues/3958
  https://github.com/whatwg/html/pull/6946
-->

<link rel="mismatch" href="support/iframe_sandbox_iframe_pdf_viewer.html">
<iframe sandbox src="support/sandbox.pdf"></iframe>
</html>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_iframe_pdf_viewer.html"
}
```

# html/webappapis/dynamic-markup-insertion/html-unsafe-methods/resources/parseHTMLUnsafe-iframe-base.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/resources/parseHTMLUnsafe-iframe-base.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>An iframe that does parseHTMLUnsafe stuff with base</title>
<base href="/fake/base-from-iframe">

<script src="/html/webappapis/dynamic-markup-insertion/html-unsafe-methods/resources/parseHTMLUnsafe-iframe.js"></script>
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
  "source_name": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/resources/parseHTMLUnsafe-iframe-base.html"
}
```

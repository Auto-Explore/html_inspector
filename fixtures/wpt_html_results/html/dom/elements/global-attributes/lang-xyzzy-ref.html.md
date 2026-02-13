# html/dom/elements/global-attributes/lang-xyzzy-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/lang-xyzzy-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Invalid languages</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<meta name="flags" content="css21">
<style>#testp { color: green; }</style>
<body>
<div id="test">
<p id="testp" lang="xyzzy">ABC</p>
</div>
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
  "source_name": "html/dom/elements/global-attributes/lang-xyzzy-ref.html"
}
```

# html/browsers/sandboxing/sandbox-parse-noscript.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-parse-noscript.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>noscript parsing when sandbox disables scripting</title>
<link rel=match href=/html/browsers/sandboxing/sandbox-parse-noscript-ref.html>
<iframe srcdoc="<noscript>PASS</noscript>" sandbox></iframe>
<iframe src="noscript-iframe.html" sandbox></iframe>
<iframe srcdoc="<noscript>P<b>AS</b>S</noscript>" sandbox></iframe>
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
  "source_name": "html/browsers/sandboxing/sandbox-parse-noscript.html"
}
```

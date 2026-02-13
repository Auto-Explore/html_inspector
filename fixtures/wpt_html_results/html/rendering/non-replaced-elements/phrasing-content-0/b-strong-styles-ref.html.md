# html/rendering/non-replaced-elements/phrasing-content-0/b-strong-styles-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/phrasing-content-0/b-strong-styles-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Test reference</title>

<div style="font: 100 32px system-ui">lighter <span style="font-weight:bolder">normal <span style="font-weight:bolder">bolder</span></span></div>
<div style="font: 32px system-ui">normal <span style="font-weight:bolder">bold <span style="font-weight:bolder">bolder</span></span></div>
<div style="font: 900 32px system-ui">context max <span style="font-weight:bolder">bolder</span></div>

<br>

<div style="font: 100 32px system-ui">lighter <span style="font-weight:bolder">normal <span style="font-weight:bolder">bolder</span></span></div>
<div style="font: 32px system-ui">normal <span style="font-weight:bolder">bold <span style="font-weight:bolder">bolder</span></span></div>
<div style="font: 900 32px system-ui">context max <span style="font-weight:bolder">bolder</span></div>
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
  "source_name": "html/rendering/non-replaced-elements/phrasing-content-0/b-strong-styles-ref.html"
}
```

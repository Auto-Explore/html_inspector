# html/rendering/non-replaced-elements/phrasing-content-0/b-strong-styles.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/phrasing-content-0/b-strong-styles.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>default styles for the b, strong elements</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#phrasing-content-3">
<link rel="help" href="https://issues.chromium.org/issues/396607231">
<link rel="author" href="mailto:dizhangg@chromium.org">
<link rel="match" href="b-strong-styles-ref.html">

<div style="font: 100 32px system-ui">lighter <b>normal <b>bolder</b></b></div>
<div style="font: 32px system-ui">normal <b>bold <b>bolder</b></b></div>
<div style="font: 900 32px system-ui">context max <b>bolder</b></div>

<br>

<div style="font: 100 32px system-ui">lighter <strong>normal <strong>bolder</strong></strong></div>
<div style="font: 32px system-ui">normal <strong>bold <strong>bolder</strong></strong></div>
<div style="font: 900 32px system-ui">context max <strong>bolder</strong></div>
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
  "source_name": "html/rendering/non-replaced-elements/phrasing-content-0/b-strong-styles.html"
}
```

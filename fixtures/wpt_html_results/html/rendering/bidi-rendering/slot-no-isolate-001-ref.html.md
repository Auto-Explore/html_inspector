# html/rendering/bidi-rendering/slot-no-isolate-001-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/bidi-rendering/slot-no-isolate-001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTML Rendering: slot element has unicode-bidi: isolate</title>
<link rel="author" title="L. David Baron" href="https://dbaron.org/">
<link rel="author" title="Google" href="http://www.google.com/">

<div style="unicode-bidi: bidi-override; direction: ltr;">&#x5D1;-&#x5D0;</div>

<div>normal</div>
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
  "source_name": "html/rendering/bidi-rendering/slot-no-isolate-001-ref.html"
}
```

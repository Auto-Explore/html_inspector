# html/dom/elements/requirements-relating-to-bidirectional-algorithm-formatting-characters/reference/dir-isolation-001-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/requirements-relating-to-bidirectional-algorithm-formatting-characters/reference/dir-isolation-001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
<meta charset="utf-8"/>
<title>The dir attribute: isolated from following number, opposite direction</title>
<style type="text/css">
.test, .ref { font-size: 150%; border: 1px solid orange; margin: 10px; margin-right: 200px; padding: 5px; clear: both; }
input { margin: 5px; }
</style>
</head>
<body>
<p class="instructions" dir="ltr">Test passes if the two boxes are identical.</p>
<div class="ref"><div dir="ltr">&#8237;&#1488; 3&#8236;</div><div dir="ltr">&#8237;a 3&#8236;</div><div dir="rtl">&#8237;3 &#1488;&#8236;</div><div dir="rtl">&#8237;3 a&#8236;</div></div>
<div class="ref"><div dir="ltr">&#8237;&#1488; 3&#8236;</div><div dir="ltr">&#8237;a 3&#8236;</div><div dir="rtl">&#8237;3 &#1488;&#8236;</div><div dir="rtl">&#8237;3 a&#8236;</div></div>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 162,
        "byte_start": 139,
        "col": 1,
        "line": 6
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
  "source_name": "html/dom/elements/requirements-relating-to-bidirectional-algorithm-formatting-characters/reference/dir-isolation-001-ref.html"
}
```

# html/dom/elements/global-attributes/dir-auto-dynamic-simple-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir-auto-dynamic-simple-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Dynamic changes and dir=auto</title>
<div>Test for elements with dir="auto" whose content changes between directional and neutral</div>
<div dir="auto">xyz</div>
<div dir="auto">ابج</div>
<div dir="auto">456</div>
<div dir="auto">xyz</div>
<div dir="auto">ابج</div>
<div dir="auto">456</div>
<div dir="auto">xyz</div>
<div dir="auto">ابج</div>
<div dir="auto">456</div>
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
  "source_name": "html/dom/elements/global-attributes/dir-auto-dynamic-simple-ref.html"
}
```

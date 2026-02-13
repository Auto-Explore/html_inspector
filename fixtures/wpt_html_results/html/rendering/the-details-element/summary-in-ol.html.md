# html/rendering/the-details-element/summary-in-ol.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/summary-in-ol.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Check if SUMMARY has 'counter-increment: list-item 0'</title>
<link rel="match" href="summary-in-ol-ref.html">

<ol>
 <details><summary>summary</summary></details>
 <li>1</li>
 <li>2 <details><summary>summary</summary></details></li>
 <li>3</li>
 <details><summary>summary</summary></details>
 <li>4</li>
</ol>

<ol>
 <summary>summary</summary>
 <li>1</li>
 <li>2 <summary>summary</summary></li>
 <li>3</li>
 <summary>summary</summary>
 <li>4</li>
</ol>

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
  "source_name": "html/rendering/the-details-element/summary-in-ol.html"
}
```

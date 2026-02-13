# html/rendering/the-details-element/details-pseudo-elements-005.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-pseudo-elements-005.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>::details-content pseudo element supports ::first-letter and ::first-line</title>
<link rel="match" href="details-pseudo-elements-005-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-details-and-summary-elements">
<link rel="help" href="https://drafts.csswg.org/css-pseudo-4/#treelike">
<link rel="help" href="https://drafts.csswg.org/css-pseudo-4/#details-content-pseudo">
<link rel="help" href="https://github.com/whatwg/html/pull/10265">
<link rel="help" href="https://github.com/dbaron/details-styling/blob/main/phase-1.md">
<link rel="help" href="https://crbug.com/1469418">
<style>

details::details-content::first-letter { font-size: 2em; }
details::details-content::first-line { font-weight: bold; }

</style>

<details open>
  <summary>summary</summary>
  The contents<br>
  are on multiple</br>
  lines.
</details>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.void_element.end_tag",
      "message": "End tag “br”.",
      "severity": "Warning",
      "span": {
        "byte_end": 870,
        "byte_start": 865,
        "col": 18,
        "line": 20
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
  "source_name": "html/rendering/the-details-element/details-pseudo-elements-005.html"
}
```

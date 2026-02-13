# html/rendering/the-details-element/details-pseudo-elements-005-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-pseudo-elements-005-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>::details-content pseudo element supports ::before and ::after</title>

<style>

.letter { font-size: 2em; }
.line { font-weight: bold; }

</style>

<details open>
  <summary>summary</summary>
  <span class="line"><span class="letter">T</span>he contents</span><br>
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
        "byte_end": 311,
        "byte_start": 306,
        "col": 18,
        "line": 14
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
  "source_name": "html/rendering/the-details-element/details-pseudo-elements-005-ref.html"
}
```

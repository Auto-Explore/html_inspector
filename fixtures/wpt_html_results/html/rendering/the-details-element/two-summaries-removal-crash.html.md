# html/rendering/the-details-element/two-summaries-removal-crash.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/two-summaries-removal-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://crbug.com/1216804">
<meta name="assert" content="The renderer should not crash.">

<details id="details" open>
  <summary></summary>
  <summary>
    <details></details>
  </summary>
</details>

<script>
details.appendChild(document.createElement("frame"));
details.innerText="";
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.multiple_summary",
      "message": "Element “summary” not allowed as child of “details” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 268,
        "byte_start": 259,
        "col": 3,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 292,
        "byte_start": 282,
        "col": 14,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/the-details-element/two-summaries-removal-crash.html"
}
```

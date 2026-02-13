# html/semantics/forms/the-datalist-element/input-text-datalist-appearance.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-datalist-element/input-text-datalist-appearance.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=assert title="text inputs should show an indicator when they have a datalist.">
<link rel=mismatch href="input-text-focused-ref.html">
<link rel=assert title="Text inputs should have different appearance when focused if they have a datalist."

<input list=mydatalist>
<datalist id=mydatalist>
  <option>option</option>
</datalist>

<script>
document.querySelector('input').focus();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 157,
        "byte_start": 68,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.lt_expecting_attr_name",
      "message": "Saw “<” when expecting an attribute name. Probable cause: Missing “>” immediately before.",
      "severity": "Warning",
      "span": {
        "byte_end": 345,
        "byte_start": 213,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 345,
        "byte_start": 213,
        "col": 1,
        "line": 5
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
  "source_name": "html/semantics/forms/the-datalist-element/input-text-datalist-appearance.html"
}
```

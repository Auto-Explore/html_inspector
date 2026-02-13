# html/semantics/interactive-elements/the-details-element/details-findstring-crash.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/details-findstring-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1264507">

<script>
window.onload = () => {
  window.getSelection().selectAllChildren(document.body);
  document.querySelector('object').remove();
  document.execCommand('FindString',false,0);
};
</script>

<details>
  <object id='id6'></object>
</details>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 379,
        "byte_start": 362,
        "col": 3,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 399,
        "byte_start": 389,
        "col": 1,
        "line": 15
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
  "source_name": "html/semantics/interactive-elements/the-details-element/details-findstring-crash.html"
}
```

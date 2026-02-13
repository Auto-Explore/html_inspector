# html/semantics/interactive-elements/the-dialog-element/dialog-open-pseudo-invalidation.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-open-pseudo-invalidation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel=help href="https://drafts.csswg.org/selectors-4/#open-state">
<link rel="match" href="dialog-open-pseudo-invalidation-ref.html">

<p>The dialog should be open and green:</p>
<dialog>Dialog</dialog>

<style>
  dialog {
    background-color: red;
  }
  dialog:open {
    background-color: green;
  }
</style>

<script>
  const d = document.querySelector('dialog');
  d.show();
  d.blur();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 255,
        "byte_start": 248,
        "col": 1,
        "line": 9
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-open-pseudo-invalidation.html"
}
```

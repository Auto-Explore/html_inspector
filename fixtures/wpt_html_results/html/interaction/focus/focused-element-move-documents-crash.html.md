# html/interaction/focus/focused-element-move-documents-crash.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focused-element-move-documents-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1172828">

<!-- No crash should occur if a focused element is moved to another document. -->

<div id=editable contenteditable=>

<script>
editable.addEventListener('blur', function() {
  document.execCommand('InsertText', false, '\t');
});
editable.focus();
const secondDoc = document.implementation.createDocument('', null);
secondDoc.appendChild(editable);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.attr_value_missing",
      "message": "Attribute value missing.",
      "severity": "Warning",
      "span": {
        "byte_end": 294,
        "byte_start": 260,
        "col": 1,
        "line": 7
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
  "source_name": "html/interaction/focus/focused-element-move-documents-crash.html"
}
```

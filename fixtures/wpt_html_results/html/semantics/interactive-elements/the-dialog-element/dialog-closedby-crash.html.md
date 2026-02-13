# html/semantics/interactive-elements/the-dialog-element/dialog-closedby-crash.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-closedby-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:luke@warlow.dev">
<link rel=author href="mailto:masonf@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/#attr-dialog-closedby">

<!-- This test passes if it does not crash. -->

<dialog id="dialog" open closedby="closerequest">Dialog</dialog>
<dialog id="dialog" open closedby="any">Dialog</dialog>
<dialog id="dialog" open closedby="none">Dialog</dialog>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “dialog”.",
      "severity": "Error",
      "span": {
        "byte_end": 345,
        "byte_start": 305,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “dialog”.",
      "severity": "Error",
      "span": {
        "byte_end": 402,
        "byte_start": 361,
        "col": 1,
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-closedby-crash.html"
}
```

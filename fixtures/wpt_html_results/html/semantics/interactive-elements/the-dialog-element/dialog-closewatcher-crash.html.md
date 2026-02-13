# html/semantics/interactive-elements/the-dialog-element/dialog-closewatcher-crash.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-closewatcher-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>

<!-- This test passes if it does not crash. -->

<dl>
  <dt contenteditable></dt>
  <dialog open></dialog>
</dl>

<script>
  const dialog = document.querySelector('dialog');
  dialog.open = false;
  document.querySelector('dl').addEventListener("focusin", () => {
    dialog.showModal();
  });
  document.defaultView.requestIdleCallback(() => {
    window.getSelection().addRange(document.createRange());
    dialog.close();
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.dl.child.invalid",
      "message": "Element “dialog” not allowed as child of “dl” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 114,
        "byte_start": 101,
        "col": 3,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.dl.missing_dd",
      "message": "Element “dl” is missing a required instance of one or more of the following child elements: “dd”.",
      "severity": "Warning",
      "span": {
        "byte_end": 129,
        "byte_start": 124,
        "col": 1,
        "line": 8
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-closewatcher-crash.html"
}
```

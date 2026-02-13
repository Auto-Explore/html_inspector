# html/semantics/interactive-elements/the-dialog-element/dialog-requestclose-4-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-requestclose-4-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="author" href="mailto:wpt@keithcirkel.co.uk" />
<link rel="help" href="https://html.spec.whatwg.org/#dom-dialog-requestclose" />

<!-- This test passes if it does not crash. -->

<script>
  const doc = document.implementation.createHTMLDocument("");
  const dialog = doc.createElement("dialog");
  dialog.setAttribute("open", "");
  doc.body.append(dialog);
  dialog.requestClose();
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-requestclose-4-crash.html"
}
```

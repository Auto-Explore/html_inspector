# html/semantics/interactive-elements/the-dialog-element/dialog-inert.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-inert.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  body { margin: 0 }
  dialog {
    width: 100%;
    height: 100%;
    max-width: 100%;
    max-height: 100%;
    box-sizing: border-box;
    padding: 0;
  }
  dialog::backdrop {
    display: none;
  }
</style>
<dialog id=dialog>Something</dialog>
<script>
test(function() {
  let dialog = document.getElementById("dialog");
  dialog.showModal();
  assert_equals(
    document.elementFromPoint(10, 10),
    dialog,
    "Dialog is hittable by default",
  );
  dialog.inert = true;
  assert_not_equals(
    document.elementFromPoint(10, 10),
    dialog,
    "Dialog becomes inert dynamically",
  );
  dialog.close();
  dialog.showModal();
  assert_not_equals(
    document.elementFromPoint(10, 10),
    dialog,
    "Dialog remains inert after open",
  );
});
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-inert.html"
}
```

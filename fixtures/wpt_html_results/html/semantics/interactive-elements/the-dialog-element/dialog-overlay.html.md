# html/semantics/interactive-elements/the-dialog-element/dialog-overlay.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-overlay.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>dialog: overlay</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<link rel="help" href="https://drafts.csswg.org/css-position-4/#overlay">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<dialog id="dialog"></dialog>
<script>
  const dialog = document.getElementById("dialog");

  test(() => {
    assert_equals(getComputedStyle(dialog).overlay, "none",
                  "Computed overlay");
    assert_equals(getComputedStyle(dialog, "::backdrop").overlay, "none",
                  "Computed overlay for ::backdrop");
  }, "dialog computed overlay initially 'none'");

  test(() => {
    dialog.showModal();

    assert_equals(getComputedStyle(dialog).overlay, "auto",
                  "Computed overlay on open dialog");
    // ::backdrop pseudo element is always rendered in the top layer when its
    // originating element is. It does not get its overlay property changed,
    // though.
    assert_equals(getComputedStyle(dialog, "::backdrop").overlay, "none",
                  "Computed overlay for ::backdrop");

    dialog.close();

    assert_equals(getComputedStyle(dialog).overlay, "none",
                  "Computed overlay on closed dialog");
    assert_equals(getComputedStyle(dialog, "::backdrop").overlay, "none",
                  "Computed overlay for ::backdrop");
  }, "Opening and closing a modal dialog changes computed overlay to 'auto' and back to 'none'");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-overlay.html"
}
```

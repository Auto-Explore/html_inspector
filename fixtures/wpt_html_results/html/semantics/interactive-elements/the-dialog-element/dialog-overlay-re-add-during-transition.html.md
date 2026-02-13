# html/semantics/interactive-elements/the-dialog-element/dialog-overlay-re-add-during-transition.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-overlay-re-add-during-transition.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<title>dialog: close and re-add modal dialog during overlay transition</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<link rel="help" href="https://drafts.csswg.org/css-position-4/#overlay">
<link rel="match" href="pass-dialog-ref.html">
<script src="/common/reftest-wait.js"></script>
<dialog id="dialog1">PASS</dialog>
<dialog id="dialog2">FAIL</dialog>
<style>
  dialog::backdrop { background-color: black; }
  #dialog1 {
    transition-property: overlay, display;
    transition-duration: 100s;
  }
</style>
<script>
  const dialog1 = document.getElementById("dialog1");
  const dialog2 = document.getElementById("dialog2");

  dialog1.showModal();
  dialog2.showModal();
  dialog1.close();
  requestAnimationFrame(() =>
    requestAnimationFrame(() => {
      // dialog1 no longer "in top layer" even if rendered in top-layer, should
      // be added as last top layer element.
      dialog1.showModal();
      takeScreenshot();
    })
  );
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
        "byte_end": 478,
        "byte_start": 471,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-overlay-re-add-during-transition.html"
}
```

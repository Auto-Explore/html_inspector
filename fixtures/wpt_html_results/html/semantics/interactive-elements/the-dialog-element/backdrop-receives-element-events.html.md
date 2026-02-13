# html/semantics/interactive-elements/the-dialog-element/backdrop-receives-element-events.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/backdrop-receives-element-events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>Test that ::backdrop receives events for the associated element</title>
<link rel="author" title="Tim Nguyen" href="https://github.com/nt1m">
<body>
<style>
/* ::backdrop takes up whole screen, actual <dialog> is hidden */
dialog {
    visibility: hidden;
    pointer-events: none;
}

dialog::backdrop {
    visibility: visible;
    pointer-events: initial;
    background-color: red;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
}

dialog.clicked::backdrop {
    background-color: green;
}
</style>
<dialog></dialog>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script>
setup({ single_test: true });

const dialog = document.querySelector("dialog");
dialog.showModal();
dialog.addEventListener("click", () => {
    // Change style for debugging purposes, done() actually makes the test pass
    dialog.className = "clicked";
    done();
});
new test_driver.Actions()
    .pointerMove(0, 0, {origin: "viewport"})
    .pointerDown()
    .pointerUp()
    .send();
</script>
</body>
</html>
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
        "byte_end": 186,
        "byte_start": 179,
        "col": 1,
        "line": 6
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/backdrop-receives-element-events.html"
}
```

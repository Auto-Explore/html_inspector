# html/semantics/interactive-elements/the-dialog-element/remove-dialog-should-unblock-document.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/remove-dialog-should-unblock-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body id="body">
    <dialog>
        This is a dialog
    </dialog>
    <input />
<script>
"use strict";
function testFocus(element, expectFocus) {
    var focusedElement = null;
    element.addEventListener('focus', function() { focusedElement = element; }, false);
    element.focus();
    var theElement = element;
    assert_equals(focusedElement === theElement, expectFocus, element.id);
}

promise_setup(async function() {
    await test_driver.click(document.documentElement);
})

promise_test(async function() {
    var dialog = document.querySelector('dialog');
    dialog.showModal();

    var input = document.querySelector('input');
    testFocus(input, false);

    dialog.remove();
    testFocus(input, true);
}, "Test that removing dialog unblocks the document.");
</script>
</body>
</html>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/remove-dialog-should-unblock-document.html"
}
```

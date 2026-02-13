# html/semantics/interactive-elements/the-dialog-element/show-modal-focusing-steps.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/show-modal-focusing-steps.html",
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
<script src="./resources/common.js"></script>
<script>
promise_test(() => {
  return waitUntilLoadedAndAutofocused().then(() => {
        outerButton = document.getElementById('outer-button');
        assert_equals(document.activeElement, outerButton);

        // Test that focus goes to the dialog if the dialog has no focusable elements
        var outerDialog = document.getElementById('outer-dialog');
        outerDialog.showModal();
        assert_equals(document.activeElement, outerDialog);

        // Test that an autofocus element in the dialog gets focus.
        var dialog = document.getElementById('dialog');
        dialog.showModal();
        autofocusButton = document.getElementById('autofocus-button');
        assert_equals(document.activeElement, autofocusButton);
        dialog.close();

        // ... or else first focusable element in the dialog gets focus.
        autofocusButton.parentNode.removeChild(autofocusButton);
        dialog.showModal();
        firstButton = document.getElementById('first-button');
        assert_equals(document.activeElement, firstButton);
        dialog.close();

        // ... or else the dialog itself gets focus.;
        var buttons = dialog.querySelectorAll('button');
        for (var i = 0; i < buttons.length; ++i)
            buttons[i].hidden = true;
        dialog.showModal();
        assert_equals(document.activeElement, dialog);
        dialog.close();

        document.getElementById('outer-dialog').close();
  });
}, "focus when a modal dialog is opened");
</script>
</head>
<body>
<button id="outer-button" autofocus></button>
<dialog id="outer-dialog">
    <dialog id="dialog" tabindex=0>
        <button disabled></button>
        <dialog>
            <button autofocus></button>
        </dialog>
        <button id="first-button"></button>
        <div>
            <span>
                <button id="autofocus-button" autofocus></button>
            </span>
        </div>
        <button id="final-button"></button>
    </dialog>
</dialog>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/show-modal-focusing-steps.html"
}
```

# html/semantics/interactive-elements/the-dialog-element/dialog-autofocus.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-autofocus.html",
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
        assert_equals(document.activeElement, document.getElementById("outer-button"));

        var dialog = document.getElementById('dialog');
        dialog.showModal();

        autofocusButton = document.getElementById('autofocus-button');
        assert_equals(document.activeElement, autofocusButton);

        anotherButton = document.getElementById('another-button');
        anotherButton.focus();
        assert_equals(document.activeElement, anotherButton);

        // Test that recreating layout does not give focus back to a previously autofocused element.
        autofocusButton.style.display = 'none';
        document.body.offsetHeight;
        autofocusButton.style.display = 'block';
        document.body.offsetHeight;
        assert_equals(document.activeElement, anotherButton);

        // Test that reinserting does not give focus back to a previously autofocused element.
        var parentNode = autofocusButton.parentNode;
        parentNode.removeChild(autofocusButton);
        document.body.offsetHeight;
        parentNode.appendChild(autofocusButton);
        document.body.offsetHeight;
        assert_equals(document.activeElement, anotherButton);

        dialog.close();
        // Test that dialog focusing steps run when a dialog is reopened.
        dialog.showModal();
        assert_equals(document.activeElement, autofocusButton);
        dialog.close();
  });
}, "autofocus when a modal dialog is opened");
</script>
</head>
<body>
<button id="outer-button" autofocus></button>
<dialog id="dialog">
    <button></button>
    <!-- Unfocusable elements with [autofocus] should be ignored. -->
    <input autofocus disabled>
    <textarea autofocus hidden></textarea>
    <dialog>
        <button autofocus></button>
    </dialog>
    <div>
        <span>
            <button id="autofocus-button" autofocus></button>
        </span>
    </div>
    <button id="another-button" autofocus></button>
</dialog>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.autofocus.multiple_in_scoping_root",
      "message": "There must not be two elements with the same \"nearest ancestor autofocus scoping root element\" that both have the “autofocus” attribute specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 2069,
        "byte_start": 2042,
        "col": 5,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.autofocus.multiple_in_scoping_root",
      "message": "There must not be two elements with the same \"nearest ancestor autofocus scoping root element\" that both have the “autofocus” attribute specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 2221,
        "byte_start": 2181,
        "col": 13,
        "line": 60
      }
    },
    {
      "category": "Html",
      "code": "html.autofocus.multiple_in_scoping_root",
      "message": "There must not be two elements with the same \"nearest ancestor autofocus scoping root element\" that both have the “autofocus” attribute specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 2300,
        "byte_start": 2262,
        "col": 5,
        "line": 63
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-autofocus.html"
}
```

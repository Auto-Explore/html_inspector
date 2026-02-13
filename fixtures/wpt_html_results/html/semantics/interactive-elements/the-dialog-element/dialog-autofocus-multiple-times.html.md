# html/semantics/interactive-elements/the-dialog-element/dialog-autofocus-multiple-times.html

Counts:
- errors: 2
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-autofocus-multiple-times.html",
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

        var focusCount = 0;
        var dlg = document.getElementById("dlg");
        var input1 = document.getElementById("input1");
        var input2 = document.getElementById("input2");
        input2.onfocus = function() { focusCount += 1 };

        var expectedFocusCount = 3;
        for (i = 0; i < expectedFocusCount; i++) {
            dlg.show();
            assert_equals(document.activeElement, input2);
            input1.focus();
            assert_equals(document.activeElement,input1);
            dlg.close();
        }

        assert_equals(focusCount.toString(), expectedFocusCount.toString());
  });
}, "autofocus is run every time a dialog is opened");
</script>
</head>
<body>
<button id="outer-button" autofocus></button>
<dialog id="dlg">
    <!-- Unfocusable elements with [autofocus] should be ignored. -->
    <input autofocus disabled>
    <textarea autofocus hidden></textarea>
    <input id="input1"></input>
    <input id="input2" autofocus></input>
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
        "byte_end": 1358,
        "byte_start": 1331,
        "col": 5,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 1401,
        "byte_start": 1393,
        "col": 24,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.autofocus.multiple_in_scoping_root",
      "message": "There must not be two elements with the same \"nearest ancestor autofocus scoping root element\" that both have the “autofocus” attribute specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 1435,
        "byte_start": 1406,
        "col": 5,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 1443,
        "byte_start": 1435,
        "col": 34,
        "line": 41
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-autofocus-multiple-times.html"
}
```

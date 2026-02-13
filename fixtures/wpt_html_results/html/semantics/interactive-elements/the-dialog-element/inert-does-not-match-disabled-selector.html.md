# html/semantics/interactive-elements/the-dialog-element/inert-does-not-match-disabled-selector.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/inert-does-not-match-disabled-selector.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
button {
    color: green;
}

button:disabled {
    color: red;
}

.trigger-style-recalc {
    /* No change, we just need a new style recalculation. */
    font-weight:bold;
}
</style>
</head>
<body style="color: green">
<button>The test passes if this is in green.</button>
<dialog></dialog>
<script>
"use strict";
test(function() {
    document.querySelector('dialog').showModal();
    var button = document.querySelector('button');
    button.classList.add('trigger-style-recalc');
    var color = document.defaultView.getComputedStyle(button).getPropertyValue('color');
    assert_equals(color, 'rgb(0, 128, 0)');
}, "Tests inert elements do not match the :disabled selector.");
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/inert-does-not-match-disabled-selector.html"
}
```

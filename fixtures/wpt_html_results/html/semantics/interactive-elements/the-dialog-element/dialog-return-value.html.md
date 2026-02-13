# html/semantics/interactive-elements/the-dialog-element/dialog-return-value.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-return-value.html",
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
</head>
<body>
<dialog></dialog>
<script>
test(function() {
    dialog = document.querySelector('dialog');
    assert_equals(dialog.returnValue, '');

    dialog.returnValue = 'Setting value directly';
    assert_equals(dialog.returnValue, 'Setting value directly');

    dialog.returnValue = null;
    assert_equals(dialog.returnValue, 'null');

    dialog.returnValue = '';
    assert_equals(dialog.returnValue, '');

    dialog.returnValue = 7;
    assert_equals(dialog.returnValue, '7');

    dialog.show();
    dialog.close('Return value set from close()');
    assert_equals(dialog.returnValue, 'Return value set from close()');

    dialog.show();
    dialog.close('');
    assert_equals(dialog.returnValue, '');

    dialog.show();
    dialog.close(null);
    assert_equals(dialog.returnValue, 'null');

    dialog.returnValue = 'Should not change because no argument to close()';
    dialog.show();
    dialog.close();
    assert_equals(dialog.returnValue, 'Should not change because no argument to close()');

    dialog.returnValue = 'Should not change because of undefined argument to close()';
    dialog.show();
    dialog.close(undefined);
    assert_equals(dialog.returnValue, 'Should not change because of undefined argument to close()');

    dialog.returnValue = 'Should not change because of no-op close()';
    dialog.close('blah');
    assert_equals(dialog.returnValue, 'Should not change because of no-op close()');
}, "Tests dialog.returnValue is settable and returns the last value set.");
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-return-value.html"
}
```

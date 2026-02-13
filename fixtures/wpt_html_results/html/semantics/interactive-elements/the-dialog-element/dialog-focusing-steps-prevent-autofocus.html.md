# html/semantics/interactive-elements/the-dialog-element/dialog-focusing-steps-prevent-autofocus.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-focusing-steps-prevent-autofocus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/interaction/focus/the-autofocus-attribute/resources/utils.js"></script>
<body>
<dialog></dialog>
<script>
// https://github.com/whatwg/html/issues/4788
promise_test(async () => {
  const dialog = document.querySelector('dialog');
  dialog.show();
  dialog.close();
  const input = document.createElement('input');
  input.autofocus = true;
  document.body.insertBefore(input, dialog);
  await waitUntilStableAutofocusState();
  assert_not_equals(document.activeElement, input,
      'Non-dialog autofocus processing should be skipped.');
}, 'After showing a dialog, non-dialog autofocus processing won\'t work.');
</script>
</body>
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-focusing-steps-prevent-autofocus.html"
}
```

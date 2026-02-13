# html/semantics/interactive-elements/the-dialog-element/dialog-autofocus-just-once.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-autofocus-just-once.html",
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
<dialog>
<input>
<input autofocus>
</dialog>
<script>
// https://github.com/whatwg/html/issues/4788
promise_test(async () => {
  const dialog = document.querySelector('dialog');
  dialog.show();
  assert_equals(document.activeElement, dialog.querySelector('[autofocus]'),
      'dialog.show() should set focus on a descendant element with an ' +
      'autofocus attribute.');
  document.activeElement.blur();
  await waitUntilStableAutofocusState();
  assert_equals(document.activeElement, document.body,
      'Non-dialog autofocus processing should be skipped.');
}, 'An autofocus element in a dialog element should not try to get focus twice.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-autofocus-just-once.html"
}
```

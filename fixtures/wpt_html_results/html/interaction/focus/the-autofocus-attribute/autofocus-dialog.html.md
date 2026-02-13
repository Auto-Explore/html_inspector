# html/interaction/focus/the-autofocus-attribute/autofocus-dialog.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/autofocus-dialog.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#the-autofocus-attribute">
<link rel='author' href='mailto:masonf@chromium.org'>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/utils.js"></script>

<body>
<script>
promise_test(async t => {
  let w = window.open('/common/blank.html');
  await waitForLoad(w);
  t.add_cleanup(() => { w.close(); });
  w.document.body.innerHTML = '<dialog><div tabindex=0 autofocus></dialog><input autofocus>';
  await waitUntilStableAutofocusState(w);
  assert_equals(w.document.activeElement.tagName, 'INPUT');
}, '<dialog> can contain autofocus, without stopping page autofocus content from working');

promise_test(async t => {
  let w = window.open('/common/blank.html');
  await waitForLoad(w);
  t.add_cleanup(() => { w.close(); });
  w.document.body.innerHTML = '<dialog><div tabindex=0 autofocus></dialog><input autofocus>';
  await waitUntilStableAutofocusState(w);
  w.document.querySelector('dialog').show();
  assert_equals(w.document.activeElement.tagName, 'DIV');
}, '<dialog>-contained autofocus element gets focused when the dialog is shown');
</script>
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/autofocus-dialog.html"
}
```

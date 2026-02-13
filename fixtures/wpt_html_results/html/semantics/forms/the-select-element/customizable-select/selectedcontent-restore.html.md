# html/semantics/forms/the-select-element/customizable-select/selectedcontent-restore.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/selectedcontent-restore.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9799">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<iframe src="resources/selectedcontent-restore-iframe.html"></iframe>

<script>
const iframe = document.querySelector('iframe');
promise_test(async () => {
  await new Promise(resolve => iframe.onload = resolve);
  await test_driver.bless();

  iframe.contentDocument.querySelector('select').value = 'two';
  assert_equals(iframe.contentDocument.querySelector('select').value, 'two',
    'Assining two to select.value should work.');
  iframe.contentDocument.querySelector('form').submit();
  await new Promise(resolve => iframe.onload = resolve);

  await test_driver.bless();
  iframe.contentWindow.history.back();
  await new Promise(resolve => iframe.onload = resolve);
  await new Promise(requestAnimationFrame);
  await new Promise(requestAnimationFrame);

  assert_equals(iframe.contentDocument.querySelector('select').value, 'two',
    'The selects value should be restored after navigating back.');
  assert_equals(iframe.contentDocument.querySelector('selectedcontent').innerHTML,
    iframe.contentDocument.querySelector('option[value=two]').innerHTML,
    'selectedcontent.innerHTML should match the selected <option>');
}, '<selectedcontent> should be up to date after form restoration.');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/selectedcontent-restore.html"
}
```

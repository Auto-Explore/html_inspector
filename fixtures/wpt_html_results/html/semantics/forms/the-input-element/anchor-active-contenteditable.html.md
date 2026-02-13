# html/semantics/forms/the-input-element/anchor-active-contenteditable.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/anchor-active-contenteditable.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">
<link rel="help" href="http://crbug.com/1007941">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<!-- This behavior is not explicitly specified. -->

<a id=anchorid href="nonexistant">anchor</a>

<script>
anchorid.addEventListener('mousedown', () => {
  anchorid.contentEditable = true;
});

promise_test(async () => {
  await test_driver.click(anchorid);
  assert_equals(document.querySelector(':active'), null);
}, 'Anchor elements should not stay :active when contentEditable is enabled.');
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
  "source_name": "html/semantics/forms/the-input-element/anchor-active-contenteditable.html"
}
```

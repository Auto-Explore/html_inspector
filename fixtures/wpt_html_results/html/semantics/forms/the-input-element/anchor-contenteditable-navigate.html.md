# html/semantics/forms/the-input-element/anchor-contenteditable-navigate.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/anchor-contenteditable-navigate.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<!-- This behavior is not explicitly specified. -->

<a id=anchorid href="javascript:window.anchornavigated = true;">anchor</a>

<script>
promise_test(async () => {
  window.anchornavigated = false;

  anchorid.contentEditable = true;
  await test_driver.click(anchorid);

  assert_false(window.anchornavigated, "Anchor's javascript: url was run.");

}, 'Anchor elements should not be able to navigate if they have contentEditable.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.a.href.invalid",
      "message": "Bad value “javascript:window.anchornavigated = true;” for attribute “href” on element “a”.",
      "severity": "Warning",
      "span": {
        "byte_end": 499,
        "byte_start": 435,
        "col": 1,
        "line": 13
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
  "source_name": "html/semantics/forms/the-input-element/anchor-contenteditable-navigate.html"
}
```

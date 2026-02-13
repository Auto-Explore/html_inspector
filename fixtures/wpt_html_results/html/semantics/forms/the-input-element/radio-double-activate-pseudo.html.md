# html/semantics/forms/the-input-element/radio-double-activate-pseudo.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/radio-double-activate-pseudo.html",
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

<input type=radio id=radioinput>

<script>
  promise_test(async () => {
    await test_driver.send_keys(radioinput, ' ');
    await test_driver.send_keys(radioinput, ' ');
    assert_equals(document.querySelector(':active'), null,
      `If the radio doesn't have the :active pseudo selector, nothing else should either.`);
  }, `<input type=radio> shouldn't have the :active pseudo element after pressing the spacebar twice.`);
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
  "source_name": "html/semantics/forms/the-input-element/radio-double-activate-pseudo.html"
}
```

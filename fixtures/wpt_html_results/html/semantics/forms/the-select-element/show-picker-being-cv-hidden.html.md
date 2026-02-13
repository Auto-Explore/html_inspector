# html/semantics/forms/the-select-element/show-picker-being-cv-hidden.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/show-picker-being-cv-hidden.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test showPicker() being rendered requirement with content-visibility</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<div style="content-visibility: hidden">
    <select id="select">
        <option>Item 1</option>
    </select>
</div>
<script>
promise_test(async t => {
    await test_driver.bless('show picker');
    assert_throws_dom('NotSupportedError', () => { select.showPicker(); });

    // Test that dynamically changing to actually being rendered works.
    await test_driver.bless('show picker');
    select.parentElement.style.contentVisibility = 'visible';
    select.showPicker();
    select.blur();
}, 'select showPicker() throws when content-visibility hidden');
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-select-element/show-picker-being-cv-hidden.html"
}
```

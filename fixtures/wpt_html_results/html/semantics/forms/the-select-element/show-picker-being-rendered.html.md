# html/semantics/forms/the-select-element/show-picker-being-rendered.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/show-picker-being-rendered.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test showPicker() being rendered requirement</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<select id="select" style="display: none;">
    <option>Item 1</option>
</select>
<script>
promise_test(async t => {
    await test_driver.bless('show picker');
    assert_throws_dom('NotSupportedError', () => { select.showPicker(); });

    // Test that dynamically changing to actually being rendered works.
    await test_driver.bless('show picker');
    select.style.display = 'inline-block';
    select.showPicker();
    select.blur();
}, 'select showPicker() throws when not being rendered');
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
  "source_name": "html/semantics/forms/the-select-element/show-picker-being-rendered.html"
}
```

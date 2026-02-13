# html/semantics/forms/the-select-element/show-picker-user-gesture.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/show-picker-user-gesture.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test showPicker() user gesture requirement</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<select>
    <option>option</option>
</select>

<script type=module>
    const select = document.querySelector('select');

    test(() => {
        assert_throws_dom('NotAllowedError', () => { select.showPicker(); });
    }, `select showPicker() requires a user gesture`);

    promise_test(async t => {
        await test_driver.bless('show picker');
        select.showPicker();
        select.blur();

        assert_false(navigator.userActivation.isActive,
            'User activation should be consumed after calling showPicker().');

        assert_throws_dom('NotAllowedError', () => select.showPicker(),
            'select.showPicker() should throw when there is no user activation.');
    }, `select showPicker() does not throw when user activation is active.`);
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
  "source_name": "html/semantics/forms/the-select-element/show-picker-user-gesture.html"
}
```

# html/semantics/forms/the-input-element/radio-focus-navigation-disabled.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/radio-focus-navigation-disabled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" href="mailto:dizhangg@chromium.org">
<link rel="help" href="http://crbug.com/427520278">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="resources/focus-utils.js"></script>

<form>
 <label><input type="radio" name="start" id="start">Outside form</label>
</form>
<form>
 <label><input type="radio" name="radio" id="a" disabled checked>disabled checked</label>
 <label><input type="radio" name="radio" id="b">enabled 1</label>
 <label><input type="radio" name="radio" id="c">enabled 2</label>
</form>
<form>
 <label><input type="radio" name="end" id="end">Outside form</label>
</form>

<script>

promise_test(async () => {
    start.focus();
    assert_equals(document.activeElement, start);
    await navigateFocusForward();
    assert_equals(document.activeElement, b);
    await navigateFocusForward();
    assert_equals(document.activeElement, end);
    await navigateFocusBackward();
    assert_equals(document.activeElement, b);
    await navigateFocusBackward();
    assert_equals(document.activeElement, start);
}, 'Should be able to tab focus in radio group even when the checked radio button is disabled.');
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
  "source_name": "html/semantics/forms/the-input-element/radio-focus-navigation-disabled.html"
}
```

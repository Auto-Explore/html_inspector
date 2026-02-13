# html/semantics/forms/the-select-element/select-clear-reappend.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-clear-reappend.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/11825">
<link rel=help href="https://issues.chromium.org/issues/444330901">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<select>
  <option>one</option>
  <option selected>two</option>
  <option>three</option>
</select>

<script>
test(() => {
  const select = document.querySelector('select');
  assert_equals(select.value, 'two',
    'two should be selected at the start of the test.');

  const options = document.querySelectorAll('option');
  select.innerHTML = '';
  options.forEach(option => {
    select.appendChild(option);
  });

  assert_equals(select.value, 'two',
    'two should still be selected after clearing and re-appending.');
}, 'select.value should stay the same after clearing and re-appending options.');
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
  "source_name": "html/semantics/forms/the-select-element/select-clear-reappend.tentative.html"
}
```

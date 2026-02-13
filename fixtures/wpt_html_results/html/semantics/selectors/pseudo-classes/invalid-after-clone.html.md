# html/semantics/selectors/pseudo-classes/invalid-after-clone.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/invalid-after-clone.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<input>
<textarea></textarea>
<script>
promise_test(async () => {
  for (let tag of ["input", "textarea"]) {
    let element = document.querySelector(tag);
    await test_driver.send_keys(element, 'something');

    assert_true(element.validity.valid, tag + ' should be valid');

    element.maxLength = 0;
    assert_true(element.matches(":invalid"), tag + ' should match :invalid');
    assert_false(element.validity.valid, tag + ' should be invalid');

    let clone = element.cloneNode(true);
    assert_true(clone.matches(":invalid"), tag + ' clone should match :invalid');
    assert_false(clone.validity.valid, tag + 'clone should be invalid');
  }
}, 'Cloned invalid inputs / textareas with interactive changes get their validity state copied correctly');
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
  "source_name": "html/semantics/selectors/pseudo-classes/invalid-after-clone.html"
}
```

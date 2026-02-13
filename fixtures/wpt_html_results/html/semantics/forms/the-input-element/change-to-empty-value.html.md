# html/semantics/forms/the-input-element/change-to-empty-value.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/change-to-empty-value.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Change event when clearing an input</title>
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1881457">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<input type="text" value="abc">
<script>
promise_test(async function() {
  let input = document.querySelector("input");
  let changeFired = false;
  input.addEventListener("change", () => {
    changeFired = true;
  }, { once: true });
  input.focus();
  assert_equals(document.activeElement, input, "Should focus input");
  assert_false(changeFired, "Shouldn't have fired change event after focus");
  input.select();
  const kBackspaceKey = "\uE003";
  await test_driver.send_keys(input, kBackspaceKey)
  assert_false(changeFired, "Shouldn't have fired change event after select");
  input.blur();
  assert_true(changeFired, "Should've have fired change event after blur");
  assert_equals(input.value, "", "Should've have cleared the value");
});
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
  "source_name": "html/semantics/forms/the-input-element/change-to-empty-value.html"
}
```

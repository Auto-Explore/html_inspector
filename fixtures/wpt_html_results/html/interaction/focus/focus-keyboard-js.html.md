# html/interaction/focus/focus-keyboard-js.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focus-keyboard-js.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1712724">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<input type=text id=text value=abc>
<script>
let text = document.getElementById("text");

document.addEventListener("keyup", function(e) {
  text.focus();
});

promise_test(async t => {
  await test_driver.send_keys(document.body, " ");

  assert_equals(document.activeElement, text, "#text should be focused by our event listener");
  assert_true(text.matches(":focus"), "#text should match :focus");
  assert_true(text.matches(":focus-visible"), "#text should match :focus-visible");
  assert_equals(text.selectionStart, text.selectionEnd, "#text should not be selected");
  assert_equals(text.value, "abc", "#text should not have changed value");
});
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
  "source_name": "html/interaction/focus/focus-keyboard-js.html"
}
```

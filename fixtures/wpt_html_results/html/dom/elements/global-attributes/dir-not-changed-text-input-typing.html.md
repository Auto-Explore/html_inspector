# html/dom/elements/global-attributes/dir-not-changed-text-input-typing.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir-not-changed-text-input-typing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>:dir() not changed by typing in a text input</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<div dir="rtl">
  <input type="text">
  <textarea></textarea>
</div>

<script>

promise_test(async () => {
  const input = document.querySelector("div[dir=rtl] > input");
  const textarea = document.querySelector("div[dir=rtl] > textarea");

  assert_true(input.matches(":dir(rtl)"), "input is RTL before text entered");
  assert_true(textarea.matches(":dir(rtl)"), "input is RTL before text entered");

  await test_driver.send_keys(input, "a");
  await test_driver.send_keys(textarea, "a");

  assert_true(input.matches(":dir(rtl)"), "input is RTL after text entered");
  assert_true(textarea.matches(":dir(rtl)"), "input is RTL after text entered");
}, "entering LTR text doesn't change directionality of a text input without dir=auto");

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
  "source_name": "html/dom/elements/global-attributes/dir-not-changed-text-input-typing.html"
}
```

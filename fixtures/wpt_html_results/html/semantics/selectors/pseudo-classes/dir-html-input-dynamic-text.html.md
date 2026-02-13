# html/semantics/selectors/pseudo-classes/dir-html-input-dynamic-text.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/dir-html-input-dynamic-text.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1622900">
<link rel="help" href="https://html.spec.whatwg.org/#the-directionality">
<input value="ltr" dir="auto">
<script>
test(function() {
  let input = document.querySelector("input");
  assert_true(input.matches(":dir(ltr)"), "Input with ltr value should match dir(ltr)");
  input.textContent = "ﷺ";
  assert_true(input.matches(":dir(ltr)"), "Should still match dir(ltr) after text change");
  input.value = "ltr2";
  assert_true(input.matches(":dir(ltr)"), "Should still match dir(ltr) after value change");
  input.value = "ﷺ";
  assert_true(input.matches(":dir(rtl)"), "Should match dir(rtl) after value change");
  input.textContent = "ltr";
  assert_true(input.matches(":dir(rtl)"), "Should match dir(rtl) after text change");
}, ":dir on <input> isn't altered by text children")
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
  "source_name": "html/semantics/selectors/pseudo-classes/dir-html-input-dynamic-text.html"
}
```

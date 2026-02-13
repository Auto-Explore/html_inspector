# html/semantics/selectors/pseudo-classes/focus-autofocus.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/focus-autofocus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:focus for autofocus)</title>
<link rel="author" title="Kent Tamura" href="mailto:tkent@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes">
<link rel=help href="https://html.spec.whatwg.org/multipage/forms.html#autofocusing-a-form-control:-the-autofocus-attribute">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
// This test can't be merged to focus.html because element.focus() may affect
// autofocus behavior.
var autofocusTest = async_test(":focus selector should work with an autofocused element.");
var input = document.createElement("input");
input.autofocus = true;
input.addEventListener("focus", function() {
  autofocusTest.step(function() {
    assert_array_equals(document.querySelectorAll(":focus"), [input])
    autofocusTest.done();
  });
}, false);
document.body.appendChild(input);
</script>
</body>
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
  "source_name": "html/semantics/selectors/pseudo-classes/focus-autofocus.html"
}
```

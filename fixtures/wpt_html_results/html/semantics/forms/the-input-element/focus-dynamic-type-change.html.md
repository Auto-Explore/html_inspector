# html/semantics/forms/the-input-element/focus-dynamic-type-change.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/focus-dynamic-type-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Input type switch on focused input shouldn't blur</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script type=module>
import inputTypes from "./input-types.js";

function tick() {
  return new Promise(resolve => {
    requestAnimationFrame(() => requestAnimationFrame(resolve));
  });
}

function test_from_to(fromType, toType) {
  if (fromType == toType) {
    return;
  }
  promise_test(async function(t) {
    const input = document.createElement("input");
    input.type = fromType;
    document.body.appendChild(input);
    input.focus();
    assert_equals(document.activeElement, input, `${fromType} input should be focused`);
    function onFocus() {
      t.assert_unreached("shouldn't be getting spurious focus events");
    }
    function onBlur() {
      t.assert_unreached("shouldn't be getting spurious blur events");
    }
    input.addEventListener("focus", onFocus);
    input.addEventListener("blur", onBlur);
    input.type = toType;
    assert_equals(document.activeElement, input, `${fromType} input should be focused after change to ${toType}`);
    assert_true(input.matches(":focus"), `${fromType} input should still match :focus`);
    assert_true(input.matches(":focus-visible"), `${fromType} input should still match :focus-visible`);
    await tick();
    assert_equals(document.activeElement, input, `${fromType} input should still be focused after change to ${toType}`);
    assert_true(input.matches(":focus"), `${fromType} input should still match :focus`);
    assert_true(input.matches(":focus-visible"), `${fromType} input should still match :focus-visible`);
    input.removeEventListener("focus", onFocus);
    input.removeEventListener("blur", onBlur);
  }, `${fromType} -> ${toType}`);
}

for (let type of inputTypes) {
  if (type == "hidden") {
    continue; // hidden inputs are not focusable
  }
  test_from_to(type, "text");
  test_from_to("text", type);
}
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
  "source_name": "html/semantics/forms/the-input-element/focus-dynamic-type-change.html"
}
```

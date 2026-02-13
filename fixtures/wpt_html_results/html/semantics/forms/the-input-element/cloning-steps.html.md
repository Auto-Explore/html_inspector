# html/semantics/forms/the-input-element/cloning-steps.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/cloning-steps.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Cloning of input elements</title>
<link rel="help" href="https://dom.spec.whatwg.org/#dom-node-clonenode">
<link rel="help" href="https://dom.spec.whatwg.org/#concept-node-clone">
<link rel="help" href="https://dom.spec.whatwg.org/#concept-node-clone-ext">
<link rel="help" href="https://html.spec.whatwg.org/multipage/forms.html#the-input-element:concept-node-clone-ext">
<link rel="author" title="Matthew Phillips" href="mailto:matthew@matthewphillips.info">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script type=module>
import inputTypes from "./input-types.js";

test(function() {
  var input = document.createElement("input");
  input.value = "foo bar";

  var copy = input.cloneNode();
  assert_equals(copy.value, "foo bar");
}, "input element's value should be cloned");

test(function() {
  var input = document.createElement("input");
  input.value = "foo bar";

  var copy = input.cloneNode();
  copy.setAttribute("value", "something else");

  assert_equals(copy.value, "foo bar");
}, "input element's dirty value flag should be cloned, so setAttribute doesn't affect the cloned input's value");

for (const inputType of inputTypes) {
  test(function() {
    var input = document.createElement("input");
    input.setAttribute("type", inputType);
    input.indeterminate = true;

    var copy = input.cloneNode();
    assert_equals(copy.indeterminate, true);
  }, `input[type=${inputType}] element's indeterminateness should be cloned`);

  test(function() {
    var input = document.createElement("input");
    input.setAttribute("type", inputType);
    input.checked = true;

    var copy = input.cloneNode();
    assert_equals(copy.checked, true);
  }, `input[type=${inputType}] element's checkedness should be cloned`);

  test(function() {
    var input = document.createElement("input");
    input.setAttribute("type", inputType);
    input.checked = false;

    var copy = input.cloneNode();
    copy.setAttribute("checked", "checked");

    assert_equals(copy.checked, false);
  }, `input[type=${inputType}] element's dirty checkedness should be cloned, so setAttribute doesn't affect the cloned input's checkedness`);
}
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
  "source_name": "html/semantics/forms/the-input-element/cloning-steps.html"
}
```

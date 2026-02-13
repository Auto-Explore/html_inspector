# html/semantics/forms/the-option-element/option-selected.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-option-element/option-selected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>HTMLOptionElement.selected</title>
<link rel=author title="Corey Farwell" href="mailto:coreyf@rwell.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#dom-option-selected">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>

<script>
test(function () {
  var elem = document.createElement("option");
  assert_equals(elem.selected, false);

  elem.setAttribute("selected", "");
  assert_equals(elem.selected, true);

  elem.removeAttribute("selected");
  assert_equals(elem.selected, false);

  elem.defaultSelected = true
  assert_equals(elem.selected, true);

  elem.defaultSelected = false;
  assert_equals(elem.selected, false);
}, "not dirty");

test(function () {
  testDirty(true);
}, "dirty, selected");

test(function () {
  testDirty(false);
}, "dirty, not selected");

function testDirty(isSelected) {
  var elem = document.createElement("option");

  elem.selected = isSelected;  // After this assignment, dirtiness=true
  assertDirty(elem, isSelected);

  elem.selected = !isSelected;  // Change the value, still dirty
  assertDirty(elem, !isSelected);
};

function assertDirty(elem, expect) {
  assert_equals(elem.selected, expect);

  elem.setAttribute("selected", "");
  assert_equals(elem.selected, expect);

  elem.removeAttribute("selected");
  assert_equals(elem.selected, expect);

  elem.defaultSelected = true;
  assert_equals(elem.selected, expect);

  elem.defaultSelected = false;
  assert_equals(elem.selected, expect);
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
  "source_name": "html/semantics/forms/the-option-element/option-selected.html"
}
```

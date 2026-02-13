# html/semantics/forms/the-select-element/select-multiple.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-multiple.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>HTMLSelectElement ask for reset</title>
<link rel="author" title="Sebastian Mayr" href="wpt@smayr.name">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<select multiple id="initial-selected">
  <option selected>Test 1</option>
  <option selected>Test 2</option>
</select>
<select multiple id="scripted-select">
  <option selected>Test 1</option>
  <option>Test 2</option>
</select>
<div id=log></div>
<script>
"use strict";

test(() => {

  const select = document.getElementById("initial-selected");
  assert_true(select.options[0].selected, "first option should be selected.");
  assert_true(select.options[1].selected, "second option should be selected.");

}, "multiple selected options exist, both set from markup");

test(() => {

  const select = document.getElementById("initial-selected");
  select.options[1].selected = true;

  assert_true(select.options[0].selected, "first option should be selected.");
  assert_true(select.options[1].selected, "second option should be selected.");

}, "multiple selected options exist, one set from script");

// crbug.com/1245443
test(() => {
  let select = document.createElement("select");
  select.length = 4;
  let o1 = select.options.item(1);
  select.multiple = true;
  select.selectedIndex = 2;
  o1.selected = true;
  select.multiple = false;
  assert_equals(select.selectedOptions.length, 1);
}, "Removing multiple attribute reduces the number of selected OPTIONs to 1");
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
  "source_name": "html/semantics/forms/the-select-element/select-multiple.html"
}
```

# html/semantics/forms/the-option-element/option-index.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-option-element/option-index.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>HTMLOptionElement.prototype.index</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-option-index">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<select>
<option id="option0">hello</option>
<option id="option1">hello</option>
</select>


<datalist>
<option id="dl-option0">hello</option>
<option id="dl-option1">hello</option>
</datalist>

<option id="doc-option0">hello</option>
<option id="doc-option1">hello</option>

<script>
"use strict";

test(() => {

  assert_equals(document.querySelector("#option0").index, 0);
  assert_equals(document.querySelector("#option1").index, 1);

}, "option index should work inside the document");

test(() => {

  assert_equals(document.querySelector("#dl-option0").index, 0);
  assert_equals(document.querySelector("#dl-option1").index, 0);

}, "option index should always be 0 for options in datalists");

test(() => {

  assert_equals(document.querySelector("#doc-option0").index, 0);
  assert_equals(document.querySelector("#doc-option1").index, 0);

}, "option index should always be 0 for options with no container");

test(() => {

  assert_equals(document.createElement("option").index, 0);
  assert_equals(document.createElement("option").index, 0);

}, "option index should always be 0 for options not even in the document");


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
  "source_name": "html/semantics/forms/the-option-element/option-index.html"
}
```

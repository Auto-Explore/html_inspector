# html/semantics/forms/the-button-element/button-type.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-type.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTMLButtonElement.prototype.type</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-button-type">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
"use strict";

test(() => {

  const button = document.createElement("button");
  assert_equals(button.type, "submit");

}, "a button's type should be submit by default");

test(() => {

  const button = document.createElement("button");

  for (const type of ["reset", "button", "submit"]) {
    button.type = type;
    assert_equals(button.type, type);

    button.type = type.toUpperCase();
    assert_equals(button.type, type);
  }

  button.type = "reset";
  button.type = "asdfgdsafd";
  assert_equals(button.type, "submit");

  button.type = "reset";
  button.type = "";
  assert_equals(button.type, "submit");

}, "a button's type should stay within the range of valid values");

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
  "source_name": "html/semantics/forms/the-button-element/button-type.html"
}
```

# html/semantics/forms/the-button-element/button-menu-historical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-menu-historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>Test that nobody implemented the now-removed menu type and attribute on button</title>
<meta charset="utf-8">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://github.com/whatwg/html/pull/2342">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<button id="b" type="menu" menu="m">button</button>
<menu id="m"></menu>

<script>
"use strict";

const button = document.querySelector("button");

test(() => {
  assert_false('menu' in button, 'The menu property must not exist on the button');
  assert_equals(button.menu, undefined, 'The value of the menu property on the button must be undefined');
}, 'button.menu, the potentially-reflecting IDL attribute, does not exist');

test(() => {
  assert_equals(button.type, 'submit', 'The type property must reflect as its invalid value default of submit');
}, 'button.type reflects properly');
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
  "source_name": "html/semantics/forms/the-button-element/button-menu-historical.html"
}
```

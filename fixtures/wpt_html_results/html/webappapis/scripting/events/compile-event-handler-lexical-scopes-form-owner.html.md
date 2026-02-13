# html/webappapis/scripting/events/compile-event-handler-lexical-scopes-form-owner.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/compile-event-handler-lexical-scopes-form-owner.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Form's lexical scope is established only for form-associated elements</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/forms.html#form-associated-element">
<link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#getting-the-current-value-of-the-event-handler">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<form id="form">
  <input onclick="window.inputOnClickElements = elements;">
  <img onclick="window.imgOnClickElements = elements;" alt="img">
  <div onclick="window.divOnClickElements = elements;">div</div>
  <x-foo onclick="window.xFooOnClickElements = elements;">x-foo</x-foo>
</form>

<script>
"use strict";

window.elements = "global_elements";

test(() => {
  const input = form.querySelector("input");
  input.click();
  assert_equals(window.inputOnClickElements, form.elements);
}, "<input> has a form owner");

test(() => {
  const img = form.querySelector("img");
  img.click();
  assert_equals(window.imgOnClickElements, form.elements);
}, "<img> has a form owner");

test(() => {
  const div = form.querySelector("div");
  div.click();
  assert_equals(window.divOnClickElements, window.elements);
}, "<div> doesn't have a form owner");

test(() => {
  customElements.define("x-foo", class extends HTMLElement {
    static formAssociated = true;
  });

  const xFoo = form.querySelector("x-foo");
  xFoo.click();
  assert_equals(window.xFooOnClickElements, form.elements);
}, "form-associated <x-foo> has a form owner");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 599,
        "byte_start": 536,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/scripting/events/compile-event-handler-lexical-scopes-form-owner.html"
}
```

# html/semantics/forms/the-form-element/form-action-reflection.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-action-reflection.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>form.action</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/forms.html#dom-fs-action">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<form id="form1" action="a.html"></form>
<form id="form2" action=""></form>
<form id="form3"></form>

<script>
"use strict";

test(() => {

  assert_equals(document.querySelector("#form1").action, (new URL("a.html", document.baseURI)).href,
    "action should equal the correct absolute URL");

}, "An action URL should be resolved relative to the document's base URL (= the document's URL in this case)");

test(() => {

  assert_equals(document.querySelector("#form2").action, document.URL);

}, "An empty-string action content attribute should cause the IDL attribute to return the document's URL (= the document's base URL in this case)");

test(() => {

  assert_equals(document.querySelector("#form3").action, document.URL);

}, "A missing action content attribute should cause the IDL attribute to return the document's URL (= the document's base URL in this case)");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.form.action.empty",
      "message": "Bad value “” for attribute “action” on element “form”.",
      "severity": "Warning",
      "span": {
        "byte_end": 402,
        "byte_start": 375,
        "col": 1,
        "line": 10
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
  "source_name": "html/semantics/forms/the-form-element/form-action-reflection.html"
}
```

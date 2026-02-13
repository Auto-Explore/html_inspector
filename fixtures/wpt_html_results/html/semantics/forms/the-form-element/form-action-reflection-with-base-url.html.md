# html/semantics/forms/the-form-element/form-action-reflection-with-base-url.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-action-reflection-with-base-url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>form.action with a base URL</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/forms.html#dom-fs-action">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<base href="/common/blank.html">

<form id="form1" action="a.html"></form>
<form id="form2" action=""></form>
<form id="form3"></form>

<script>
"use strict";

test(() => {

  assert_equals(document.querySelector("#form1").action, (new URL("a.html", document.baseURI)).href,
    "action should equal the correct absolute URL");

}, "An action URL should be resolved relative to the document's base URL (not the document's URL)");

test(() => {

  assert_equals(document.querySelector("#form2").action, document.URL);

}, "An empty-string action content attribute should cause the IDL attribute to return the document's URL (not the document's base URL)");

test(() => {

  assert_equals(document.querySelector("#form3").action, document.URL);

}, "A missing action content attribute should cause the IDL attribute to return the document's URL (not the document's base URL)");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 381,
        "byte_start": 349,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.form.action.empty",
      "message": "Bad value “” for attribute “action” on element “form”.",
      "severity": "Warning",
      "span": {
        "byte_end": 451,
        "byte_start": 424,
        "col": 1,
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
  "source_name": "html/semantics/forms/the-form-element/form-action-reflection-with-base-url.html"
}
```

# html/semantics/forms/the-textarea-element/value-defaultValue-textContent-xhtml.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/value-defaultValue-textContent-xhtml.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>textarea element value/defaultValue/textContent functionality</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me"/>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-textarea-value"/>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script>
"use strict";

test(() => {

  const textarea = document.createElement("textarea");

  textarea.appendChild(document.createCDATASection("foo bar baz"));
  assert_equals(textarea.defaultValue, "foo bar baz", "the defaultValue should reflect the textContent");
  assert_equals(textarea.value, "foo bar baz",
    "changing the child text content should change the raw value, and subsequently the api value");

}, "defaultValue and value include CDATASection Text nodes");
</script>
</body>
</html>
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
  "source_name": "html/semantics/forms/the-textarea-element/value-defaultValue-textContent-xhtml.xhtml"
}
```

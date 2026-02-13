# html/dom/elements/global-attributes/id-name-specialcase.html

Counts:
- errors: 5
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/id-name-specialcase.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
﻿<!DOCTYPE html>
<title>HTML5: test id with none pure alpha characters </title>
<link rel="author" title="justin.shen" href=mailto:cosmichut@msn.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div style="display:none">
  <input id="123" value="123"></input>
  <input id="1test" value="1test"></input>
  <input id="_test" value="_test"></input>
  <input id="." value="."></input>
  <input id="中国" value="china"></input>
</div>
<script>
test(function() {
  assert_equals(document.getElementById("123").value, "123");
}, "id with digits only");
test(function() {
  assert_equals(document.getElementById("1test").value, "1test");
},"id start with digits");
test(function() {
  assert_equals(document.getElementById("_test").value, "_test");
},"id start with underscore");
test(function() {
  assert_equals(document.getElementById(".").value, ".");
},"id with punctuation only");
test(function() {
  assert_equals(document.getElementById("中国").value, "china");
},"id with chinese character");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.quote_in_unquoted",
      "message": "“\"” in an unquoted attribute value. Probable causes: Attributes running together or a URL query string in an unquoted attribute value.",
      "severity": "Warning",
      "span": {
        "byte_end": 152,
        "byte_start": 82,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 345,
        "byte_start": 337,
        "col": 31,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 388,
        "byte_start": 380,
        "col": 35,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 431,
        "byte_start": 423,
        "col": 35,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 466,
        "byte_start": 458,
        "col": 27,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 510,
        "byte_start": 502,
        "col": 36,
        "line": 12
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
  "source_name": "html/dom/elements/global-attributes/id-name-specialcase.html"
}
```

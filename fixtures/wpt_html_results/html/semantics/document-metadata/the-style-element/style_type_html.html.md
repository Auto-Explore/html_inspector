# html/semantics/document-metadata/the-style-element/style_type_html.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style_type_html.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>&lt;style> type="" edge cases</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#update-a-style-block">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
#test1 { color: rgb(0, 128, 0); }
</style>

<style type="">
#test2 { color: rgb(0, 128, 0); }
</style>

<style type="TEXT/CsS">
#test3 { color: rgb(0, 128, 0); }
</style>

<style type=" text/css ">
#test4 { color: rgb(0, 128, 0); }
</style>

<style type="text/css; charset=utf-8">
#test5 { color: rgb(0, 128, 0); }
</style>

<body>

<div id="test1"></div>
<div id="test2"></div>
<div id="test3"></div>
<div id="test4"></div>
<div id="test5"></div>

<script>
"use strict";

test(() => {
  assertApplied("test1");
}, "With no type attribute, the style should apply");

test(() => {
  assertApplied("test2");
}, "With an empty type attribute, the style should apply");

test(() => {
  assertApplied("test3");
}, "With a mixed-case type attribute, the style should apply");

test(() => {
  assertNotApplied("test4");
}, "With a whitespace-surrounded type attribute, the style should not apply");

test(() => {
  assertNotApplied("test5");
}, "With a charset parameter in the type attribute, the style should not apply");

function getColor(id) {
  return window.getComputedStyle(document.getElementById(id)).color;
}

function assertApplied(id) {
  assert_equals(getColor(id), "rgb(0, 128, 0)");
}

function assertNotApplied(id) {
  assert_not_equals(getColor(id), "rgb(0, 128, 0)");
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.text_css_only",
      "message": "The only allowed value for the “type” attribute for the “style” element is “text/css” (with no parameters). (But the attribute is not needed and should be omitted altogether.)",
      "severity": "Warning",
      "span": {
        "byte_end": 430,
        "byte_start": 415,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 498,
        "byte_start": 475,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 568,
        "byte_start": 543,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.text_css_only",
      "message": "The only allowed value for the “type” attribute for the “style” element is “text/css” (with no parameters). (But the attribute is not needed and should be omitted altogether.)",
      "severity": "Warning",
      "span": {
        "byte_end": 651,
        "byte_start": 613,
        "col": 1,
        "line": 25
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
  "source_name": "html/semantics/document-metadata/the-style-element/style_type_html.html"
}
```

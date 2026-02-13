# html/semantics/scripting-1/the-template-element/additions-to-parsing-xhtml-documents/tag-name.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/additions-to-parsing-xhtml-documents/tag-name.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <title>tagName in template</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
<template><div></div></template>
<div id="log"></div>
<script><![CDATA[
test(function() {
  assert_equals(document.getElementsByTagName("template")[0].content.firstChild.tagName, 'div', "tagName case");
}, "tagName case");
]]></script>
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
  "source_name": "html/semantics/scripting-1/the-template-element/additions-to-parsing-xhtml-documents/tag-name.xhtml"
}
```

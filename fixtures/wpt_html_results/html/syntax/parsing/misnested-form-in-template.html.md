# html/syntax/parsing/misnested-form-in-template.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/misnested-form-in-template.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" href="mailto:haoran.tang.personal@gmail.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div>
  A<template><br>BC<form>D<div>E</form>F</div>G</template>H
</div>

<script>
test(function() {
  const wrapper = document.querySelector('div');
  const expectedContent = "A<template><br>BC<form>D<div>E</div></form>FG</template>H";
  const actualContent = wrapper.innerHTML.trim();
  assert_equals(actualContent, expectedContent, "The parsed structure should match the expected result with correctly nested elements.");
}, "Testing parsing of misnested <form> tags inside <template>");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “template”.",
      "severity": "Error",
      "span": {
        "byte_end": 251,
        "byte_start": 240,
        "col": 48,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “div”.",
      "severity": "Error",
      "span": {
        "byte_end": 259,
        "byte_start": 253,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/syntax/parsing/misnested-form-in-template.html"
}
```

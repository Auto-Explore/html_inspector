# html/semantics/forms/textfieldselection/textarea-selection-while-parsing.xhtml

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/textfieldselection/textarea-selection-while-parsing.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<body>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<textarea>
a
<script>document.querySelector('textarea').value = 'ggg';</script>
b
</textarea>
<script>
test(() => {
  let ta = document.querySelector('textarea');
  assert_equals(ta.selectionStart, 3);
  assert_equals(ta.selectionEnd, 3);
}, 'Value setter while parsing textarea children should move ' +
    'selection{Start,End} to the end');
</script>
</body>
</html>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/forms/textfieldselection/textarea-selection-while-parsing.xhtml"
}
```

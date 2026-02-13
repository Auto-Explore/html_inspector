# html/semantics/forms/textfieldselection/setSelectionRange.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/textfieldselection/setSelectionRange.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title></title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<textarea>

</textarea>
<script>
test(function() {
    let textarea = document.querySelector('textarea');
    assert_equals(textarea.selectionStart, 0);
    assert_equals(textarea.selectionEnd, 0);
    textarea.setSelectionRange(0, 1);
    assert_equals(textarea.selectionStart, 0);
    assert_equals(textarea.selectionEnd, 1);
}, "setSelectionRange on line boundaries");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 46,
        "byte_start": 39,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/forms/textfieldselection/setSelectionRange.html"
}
```

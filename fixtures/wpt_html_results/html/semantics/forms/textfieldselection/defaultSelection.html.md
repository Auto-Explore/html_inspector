# html/semantics/forms/textfieldselection/defaultSelection.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/textfieldselection/defaultSelection.html",
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
<textarea>foo</textarea>
<input type="text" value="foo"></input>
<script>

for (let el of [document.querySelector("textarea"), document.querySelector("input")]) {
    test(function() {
        assert_equals(el.selectionStart, 0);
        assert_equals(el.selectionEnd, 0);
    }, `Default selectionStart and selectionEnd for ${el}`);

    test(function() {
        el.value="foo";
        assert_equals(el.selectionStart, 0);
        assert_equals(el.selectionEnd, 0);
    }, `selectionStart and selectionEnd do not change when same value set again for ${el}`);

    test(function() {
        el.value="Foo";
        assert_equals(el.selectionStart, 3);
        assert_equals(el.selectionEnd, 3);
    }, `selectionStart and selectionEnd change when value changed to upper case for ${el}`);
  }
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
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 225,
        "byte_start": 217,
        "col": 32,
        "line": 7
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
  "source_name": "html/semantics/forms/textfieldselection/defaultSelection.html"
}
```

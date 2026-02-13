# html/semantics/forms/constraints/infinite_backtracking.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/constraints/infinite_backtracking.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>The infinite pattern validation test</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<input type=text id=badinput value="12345678901234567890123456789123456789z" pattern="(\d+)*$">
<script>
  test(function(){
    var elements = document.querySelectorAll(":invalid");
    assert_array_equals(elements, [document.getElementById('badinput')]);
  }, "Infinite backtracking pattern terminates");
</script>
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
  "source_name": "html/semantics/forms/constraints/infinite_backtracking.tentative.html"
}
```

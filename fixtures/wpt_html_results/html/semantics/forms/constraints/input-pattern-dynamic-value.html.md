# html/semantics/forms/constraints/input-pattern-dynamic-value.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/constraints/input-pattern-dynamic-value.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Pattern dynamic value attribute change</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1636495">
<input pattern="a" value="a">
<script>
test(function() {
  let i = document.querySelector("input");
  assert_false(i.matches(":invalid"));
  i.pattern = "b";
  assert_true(i.matches(":invalid"));
  i.pattern = "(";
  assert_false(i.matches(":invalid"));
}, "input validation is updated after pattern attribute change");
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
  "source_name": "html/semantics/forms/constraints/input-pattern-dynamic-value.html"
}
```

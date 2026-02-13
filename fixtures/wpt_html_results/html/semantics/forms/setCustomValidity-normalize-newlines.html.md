# html/semantics/forms/setCustomValidity-normalize-newlines.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/setCustomValidity-normalize-newlines.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<form>
  <input id="email" required>
</form>
<script>
test(function() {
  const input = document.getElementById("email");
  input.setCustomValidity("First line\rSecond line\r\nThird line\nFourth line");
  assert_equals(input.validationMessage, "First line\nSecond line\nThird line\nFourth line");
}, "setCustomValidity should normalize newlines from the given error message");
</script>
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
  "source_name": "html/semantics/forms/setCustomValidity-normalize-newlines.html"
}
```

# html/semantics/scripting-1/the-script-element/script-onload-string.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-onload-string.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Script: setting onload to a string</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
test(function() {
  var s = document.createElement("script");
  assert_equals(s.onload, null);
  var dummy = function() {};
  s.onload = dummy;
  assert_equals(s.onload, dummy);
  s.onload = "w('load DOM appended')";
  assert_equals(s.onload, null);
}, "Setting onload to a string should convert to null.");
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-onload-string.html"
}
```

# html/semantics/forms/the-option-element/option-text-label.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-option-element/option-text-label.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>HTMLOptionElement.text</title>
<link rel=author title=Ms2ger href="mailto:Ms2ger@gmail.com">
<link rel=help href="https://html.spec.whatwg.org/multipage/#dom-option-text">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
test(function() {
  var option = document.createElement("option");
  option.setAttribute("label", "label");
  option.textContent = "text";
  assert_equals(option.text, "text");
}, "Option with non-empty label.");

test(function() {
  var option = document.createElement("option");
  option.setAttribute("label", "");
  option.textContent = "text";
  assert_equals(option.text, "text");
}, "Option with empty label.");
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
  "source_name": "html/semantics/forms/the-option-element/option-text-label.html"
}
```

# html/interaction/focus/sequential-focus-navigation-and-the-tabindex-attribute/focus-tabindex-default-value.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/sequential-focus-navigation-and-the-tabindex-attribute/focus-tabindex-default-value.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focus - default value of tabindex</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#sequential-focus-navigation-and-the-tabindex-attribute">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<button id="test1">TEST1</button>
<div id="test2">TEST2</div>
<script>

test(function() {
  assert_equals(document.getElementById("test1").tabIndex, 0, "The value of tabIndex attribute should be 0.");
}, "The default value of tabIndex attribute must be 0 for elements that are focusable");

test(function() {
  assert_equals(document.getElementById("test2").tabIndex, -1, "The value of tabIndex attribute should be -1.");
}, "The default value of tabIndex attribute must be -1 for elements that are not focusable");

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
  "source_name": "html/interaction/focus/sequential-focus-navigation-and-the-tabindex-attribute/focus-tabindex-default-value.html"
}
```

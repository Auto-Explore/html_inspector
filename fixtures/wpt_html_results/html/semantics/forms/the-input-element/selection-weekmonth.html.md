# html/semantics/forms/the-input-element/selection-weekmonth.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/selection-weekmonth.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>Input element programmatic selection support</title>
<link rel="author" title="yaycmyk" href="mailto:evan@yaycmyk.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/forms.html#dom-textarea/input-select">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>

/* all textual, non-hidden inputs support .select() */
test(function() {
  var valid = [
    "month",
    "week",
  ];

  valid.forEach(function(type) {
    test(function() {
      var input = document.createElement("input");
      var a;

      input.type = type;
      assert_equals(input.type, type, "the given input type is not supported");

      input.select();

    }, "input type " + type + " should support the select() method");
  });
});

/* only certain input types are allowed to have a variable-length selection */
test(function() {
  var invalid = [
    "month",
    "week",
  ];

  invalid.forEach(function(type) {
    test(function() {
      var input = document.createElement("input");

      input.type = type;
      assert_equals(input.type, type, "the given input type is not supported");

      assert_equals(input.selectionStart, null, 'getting input.selectionStart');
      assert_throws_dom("INVALID_STATE_ERR", function() { input.selectionStart = 0; });
      assert_equals(input.selectionEnd, null, 'getting input.selectionEnd');
      assert_throws_dom("INVALID_STATE_ERR", function() { input.selectionEnd = 0; });
      assert_equals(input.selectionDirection, null, 'getting input.selectionDirection');
      assert_throws_dom("INVALID_STATE_ERR", function() { input.selectionDirection = "none"; });
      assert_throws_dom("INVALID_STATE_ERR", function() { input.setSelectionRange(0, 0); });
      assert_throws_dom("INVALID_STATE_ERR", function() { input.setRangeText('', 0, 0); });

    }, "input type " + type + " should not support variable-length selections");
  });
});
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
  "source_name": "html/semantics/forms/the-input-element/selection-weekmonth.html"
}
```

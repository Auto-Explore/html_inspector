# html/semantics/forms/the-input-element/hidden.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/hidden.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Hidden input element</title>
    <link rel="author" title="Kinuko Yasuda" href="mailto:kinuko@chromium.org">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#hidden-state-(type=hidden)">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>

  <body>
    <h1>Hidden input element</h1>
    <div style="display: none">

    <input id="hidden" type="hidden" />
    <input id="hidden_with_value" type="hidden" value="foo" />

    </div>
    <div id="log"></div>
  <script type="text/javascript">

    test(
      function() {
        assert_equals(document.getElementById("hidden").value, "");
        assert_equals(document.getElementById("hidden_with_value").value, "foo");
      }, "Value returns the current value for hidden");

    test(
      function() {
        document.getElementById("hidden").value = "A";
        assert_equals(document.getElementById("hidden").value, "A");
        document.getElementById("hidden").value = "B";
        assert_equals(document.getElementById("hidden").value, "B");
      }, "Setting value changes the current value for hidden");

    test(
      function() {
        assert_equals(document.getElementById("hidden").files, null);
      }, "files attribute must return null for hidden");

    test(
      function() {
        assert_equals(document.getElementById("hidden").valueAsDate, null);
      }, "valueAsDate attribute must return null for hidden");

    test(
      function() {
        assert_equals(document.getElementById("hidden").valueAsNumber, NaN);
      }, "valueAsNumber attribute must return NaN for hidden");

    test(
      function() {
        assert_equals(document.getElementById("hidden").list, null);
      }, "list attribute must return null for hidden");

    test(
      function() {
        var el = document.getElementById("hidden");
        assert_throws_dom("InvalidStateError", function() { el.stepDown(); }, "");
      }, "stepDown does not apply for hidden");

    test(
      function() {
        var el = document.getElementById("hidden");
        assert_throws_dom("InvalidStateError", function() { el.stepUp(); }, "");
      }, "stepUp does not apply for hidden");

    test(function(){
      var el = document.getElementById("hidden");
      assert_false(el.willValidate);
    }, "input type=hidden is barred from constraint validation");
  </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 622,
        "byte_start": 591,
        "col": 3,
        "line": 20
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
  "source_name": "html/semantics/forms/the-input-element/hidden.html"
}
```

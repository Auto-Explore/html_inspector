# html/semantics/forms/constraints/inputwillvalidate.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/constraints/inputwillvalidate.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html><head>
    <title>willValidate property on the input element</title>
    <meta content="text/html; charset=UTF-8" http-equiv="content-type">
    <meta content="willValidate property on the input element" name="description">
    <link href="https://html.spec.whatwg.org/multipage/#dom-cva-willvalidate" rel="help">
  </head>
  <body>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>

    <div id="log"></div>

    <form action="http://www.example.com/" style="display : none">
      <input required="" type="text">
      <input disabled="" type="text">
    </form>

    <script type="text/javascript">

      test(function() {assert_true(document.getElementsByTagName("input")[0].willValidate)}, "willValidate property returns true when required attribute exists");
      test(function() {assert_false(document.getElementsByTagName("input")[1].willValidate)}, "willValidate property returns false when disabled attribute exists");

    </script>

</body></html>
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
        "byte_end": 687,
        "byte_start": 656,
        "col": 5,
        "line": 19
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
  "source_name": "html/semantics/forms/constraints/inputwillvalidate.html"
}
```

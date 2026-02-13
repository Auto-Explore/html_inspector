# html/semantics/forms/the-progress-element/progress-2.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-progress-element/progress-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>Progress Element Tests</title>
    <meta http-equiv="content-type" content="text/html; charset=UTF-8" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-progress-element" />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <progress id="p00" style="display: none"></progress>
    <progress id="p01" max="5.5" value=".5" style="display: none"></progress>
    <script>
      test(function () {
        assert_equals(document.getElementById('p00').position, -1);
      }, "progress position equals -1");

      test(function () {
        assert_equals(document.getElementById('p00').value, 0);
      }, "progress value equals 0");

      test(function () {
        assert_equals(document.getElementById('p01').value, .5);
      }, "progress value equals .5");

      test(function () {
        document.getElementById('p00').value = document.getElementById('p00').value;
        assert_equals(document.getElementById('p00').position, 0);
      }, "progress position equals 0");
    </script>
  </body>
</html>
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
  "source_name": "html/semantics/forms/the-progress-element/progress-2.html"
}
```

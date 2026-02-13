# html/syntax/parsing/html5lib_innerHTML_template.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/html5lib_innerHTML_template.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf8">
    <title>HTML 5 Parser tests html5lib_innerHTML_template.html</title>
    <meta name="timeout" content="long">
  </head>
  <body>
    <h1>html5lib Parser Test</h1>
    <div id="log"></div>
    <script src="common.js"></script>
    <script src="test.js"></script>
    <script src="template.js"></script>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script>
      var num_iframes = 8;
      var order = ['64d3e0e4395745b6ea928e5f5cf888bf675a598b',];
      var tests = {
          "64d3e0e4395745b6ea928e5f5cf888bf675a598b":[async_test('html5lib_innerHTML_template.html 64d3e0e4395745b6ea928e5f5cf888bf675a598b'), "%3Ctemplate%3E%3Cform%3E%3Cinput%20name%3D%22q%22%3E%3C/form%3E%3Cdiv%3Esecond%3C/div%3E%3C/template%3E", "%23document%0A%7C%20%3Ctemplate%3E%0A%7C%20%20%20content%0A%7C%20%20%20%20%20%3Cform%3E%0A%7C%20%20%20%20%20%20%20%3Cinput%3E%0A%7C%20%20%20%20%20%20%20%20%20name%3D%22q%22%0A%7C%20%20%20%20%20%3Cdiv%3E%0A%7C%20%20%20%20%20%20%20%22second%22", 'template'],
      }
      init_tests("innerHTML");
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
  "source_name": "html/syntax/parsing/html5lib_innerHTML_template.html"
}
```

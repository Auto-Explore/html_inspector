# html/semantics/tabular-data/the-caption-element/caption_001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-caption-element/caption_001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>HTML5 Table API Tests</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type">
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-caption-element" />
  </head>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <body>
    <div id="log"></div>
    <table id="table1" style="display:none">
      <tr><td></td></tr>
      <caption>first caption</caption>
      <caption>second caption</caption>
    </table>
    <table id="table2" style="display:none">
      <tr><td></td></tr>
    </table>
    <table id="table3" style="display:none">
      <tr><td></td></tr>
    </table>
    <table id="table4" style="display:none">
      <tr><td></td></tr>
      <caption>first caption</caption>
    </table>
    <script>
      test(function () {
        assert_equals(document.getElementById('table1').caption.innerHTML, "first caption");
      }, "first caption element child of the first table element");

      test(function () {
        var caption = document.createElement("caption");
        caption.innerHTML = "new caption";
        var table = document.getElementById('table1');
        table.caption = caption;

        assert_equals(caption.parentNode, table);
        assert_equals(table.firstChild, caption);
        assert_equals(table.caption.innerHTML, "new caption");

        captions = table.getElementsByTagName('caption');
        assert_equals(captions.length, 2);
        assert_equals(captions[0].innerHTML, "new caption");
        assert_equals(captions[1].innerHTML, "second caption");
      }, "setting caption on a table");

      test(function () {
        assert_equals(document.getElementById('table2').caption, null);
      }, "caption IDL attribute is null");

      test(function () {
        var table = document.getElementById('table3');
        var caption = document.createElement("caption")
        table.rows[0].appendChild(caption);
        assert_equals(table.caption, null);
      }, "caption of the third table element should be null");

      test(function () {
        assert_not_equals(document.getElementById('table4').caption, null);

        var parent = document.getElementById('table4').caption.parentNode;
        parent.removeChild(document.getElementById('table4').caption);

        assert_equals(document.getElementById('table4').caption, null);
      }, "dynamically removing caption on a table");
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
  "source_name": "html/semantics/tabular-data/the-caption-element/caption_001.html"
}
```

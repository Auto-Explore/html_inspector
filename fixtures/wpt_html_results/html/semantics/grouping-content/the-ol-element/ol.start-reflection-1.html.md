# html/semantics/grouping-content/the-ol-element/ol.start-reflection-1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/ol.start-reflection-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
 <head>
  <title>ol.start - reflection test</title>
  <link rel="author" title="Shiki Okasaka" href="http://shiki.esrille.com/">
  <link rel="author" title="Esrille Inc." href="http://www.esrille.com/">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-ol-element">
  <meta name="assert" content="This test checks that the start IDL attribute reflects the respective content attribute of the same name.">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
  <ol id="ol">
   <li>One</li>
   <li>Two</li>
   <li>Three</li>
  </ol>
  <div id="log"></div>
  <script>
test(function() {
    assert_equals(document.getElementById('ol').start, 1);
})
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
  "source_name": "html/semantics/grouping-content/the-ol-element/ol.start-reflection-1.html"
}
```

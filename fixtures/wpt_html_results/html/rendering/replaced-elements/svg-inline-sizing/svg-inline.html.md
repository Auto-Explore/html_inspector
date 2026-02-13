# html/rendering/replaced-elements/svg-inline-sizing/svg-inline.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/svg-inline-sizing/svg-inline.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>SVG sizing: inline</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="../resources/svg-sizing.js"></script>
    <style>
      #testContainer {
          position: absolute;
          left: 0;
          top: 0;
          width: 800px;
          height: 600px;
      }
    </style>
    <link rel="help" href="http://www.w3.org/TR/CSS2/visudet.html#inline-replaced-width">
    <link rel="help" href="http://www.w3.org/TR/CSS2/visudet.html#inline-replaced-height">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#replaced-elements">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-dim-width">
    <link rel="help" href="http://www.w3.org/TR/SVG/coords.html#ViewportSpace">
  </head>
  <body>
    <div id="log"></div>
    <div id="testContainer"></div>
    <div id="demo"></div>
    <script src="svg-inline.js"></script>
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
  "source_name": "html/rendering/replaced-elements/svg-inline-sizing/svg-inline.html"
}
```

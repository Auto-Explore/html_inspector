# html/editing/dnd/datastore/015-manual.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/datastore/015-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Using dataTransfer in new thread</title>
    <style type="text/css">
blockquote { height: 100px; width: 100px; background: orange; margin: 0; padding: 0; float: left; }
blockquote + blockquote { background: blue; }
blockquote + blockquote + blockquote { background: fuchsia; }
blockquote + div { clear: left; }
    </style>
    <script type="text/javascript" src="/resources/testharness.js"></script>
    <script type="text/javascript" src="/resources/testharnessreport.js"></script>
    <script type="text/javascript">
setup(function () {},{explicit_done:true,explicit_timeout:true});
window.onload = function () {

  var orange  = document.getElementsByTagName('blockquote')[0],
      blue    = document.getElementsByTagName('blockquote')[1],
      fuchsia = document.getElementsByTagName('blockquote')[2],
      evtdone = {};

  orange.ondragstart = function (e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text','dragstart real data');
    var dataTransfer = e.dataTransfer;
    setTimeout(function () {
      test(function () {
        assert_equals( dataTransfer.getData('text'), '', 'step 1' );
        dataTransfer.setData('text','new thread after dragstart');
        assert_equals( dataTransfer.getData('text'), '', 'step 2' );
      },'dragstart data store should be protected after new thread starts');
    },0);
  };

  fuchsia.ondragenter = fuchsia.ondragover = function (e) {
    e.preventDefault();
  };

  fuchsia.ondrop = function (e) {
    e.preventDefault();
    var dataTransfer = e.dataTransfer;
    setTimeout(function () {
      test(function () {
        assert_equals( dataTransfer.getData('text'), '', 'step 1' );
        dataTransfer.setData('text','new thread after dragstart');
        assert_equals( dataTransfer.getData('text'), '', 'step 2' );
      },'drop data store should be protected after new thread starts');
      done();
    },0);
  };

};
    </script>
  </head>
  <body>
    <p>Drag the orange square over the blue square then the fuchsia square, then release it.</p>
    <blockquote draggable="true"></blockquote>
    <blockquote></blockquote>
    <blockquote></blockquote>
    <div id="log"></div>
    <noscript><p>Enable JavaScript and reload</p></noscript>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 111,
        "byte_start": 88,
        "col": 5,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 434,
        "byte_start": 371,
        "col": 5,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 517,
        "byte_start": 448,
        "col": 5,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 562,
        "byte_start": 531,
        "col": 5,
        "line": 13
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
  "source_name": "html/editing/dnd/datastore/015-manual.html"
}
```

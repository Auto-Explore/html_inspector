# html/editing/dnd/target-origin/002-manual.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/002-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>allowTargetOrigin events</title>
    <style type="text/css">
blockquote { height: 100px; width: 100px; background: orange; margin: 0; padding: 0; float: left; }
blockquote + blockquote { background: blue; }
blockquote + blockquote + blockquote { background: fuchsia; }
blockquote + div { clear: left; }
    </style>
    <script type="text/javascript" src="/resources/testharness.js"></script>
    <script type="text/javascript" src="/resources/testharnessreport.js"></script>
    <script type="text/javascript">
setup(function () {},{explicit_done:true});
window.onload = function () {
  var orange  = document.getElementsByTagName('blockquote')[0],
      blue    = document.getElementsByTagName('blockquote')[1],
      fuchsia = document.getElementsByTagName('blockquote')[2],
      evtdone = {};
  orange.ondragstart = function (e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text','dummy text');
    if( evtdone[e.type] ) { return; }
    evtdone[e.type] = true;
    test(function() {
      assert_true( !!e.dataTransfer.allowTargetOrigin );
    }, 'allowTargetOrigin should exist in '+e.type );
    test(function() {
      e.dataTransfer.allowTargetOrigin('*');
    }, 'allowTargetOrigin should work in '+e.type );
  };
  blue.ondragenter = blue.ondragover = function (e) {
    e.preventDefault();
  };
  orange.ondrag = blue.ondragleave = function (e) {
    if( evtdone[e.type] ) { return; }
    evtdone[e.type] = true;
    test(function() {
      assert_true( !!e.dataTransfer.allowTargetOrigin );
    }, 'allowTargetOrigin should exist in '+e.type );
    test(function() {
      assert_throws_dom( 'SECURITY_ERR', function () { e.dataTransfer.allowTargetOrigin('*'); } );
    }, 'allowTargetOrigin should throw a SECURITY_ERR in '+e.type );
  };
  fuchsia.ondragenter = fuchsia.ondragover = fuchsia.ondrop = function (e) {
    e.preventDefault();
    if( evtdone[e.type] ) { return; }
    evtdone[e.type] = true;
    test(function() {
      assert_true( !!e.dataTransfer.allowTargetOrigin );
    }, 'allowTargetOrigin should exist in '+e.type );
    test(function() {
      assert_throws_dom( 'SECURITY_ERR', function () { e.dataTransfer.allowTargetOrigin('*'); } );
    }, 'allowTargetOrigin should throw a SECURITY_ERR in '+e.type );
  };
  orange.ondragend = function (e) {
    if( evtdone[e.type] ) { return; }
    evtdone[e.type] = true;
    test(function() {
      assert_true( !!e.dataTransfer.allowTargetOrigin );
    }, 'allowTargetOrigin should exist in '+e.type );
    test(function() {
      assert_throws_dom( 'SECURITY_ERR', function () { e.dataTransfer.allowTargetOrigin('*'); } );
    }, 'allowTargetOrigin should throw a SECURITY_ERR in '+e.type );
    test(function() {
      var failtxt = '- Reload and try again';
      assert_true( evtdone.dragstart, 'dragstart event was not tested'+failtxt );
      assert_true( evtdone.drag, 'drag event was not tested'+failtxt );
      assert_true( evtdone.dragenter, 'dragenter event was not tested'+failtxt );
      assert_true( evtdone.dragleave, 'dragleave event was not tested'+failtxt );
      assert_true( evtdone.dragover, 'dragover event was not tested'+failtxt );
      assert_true( evtdone.drop, 'drop event was not tested'+failtxt );
      assert_true( evtdone.dragend, 'dragend event was not tested'+failtxt );
    }, 'all event types must now have been tested' );
    done();
  };
};
    </script>
  </head>
  <body>
    <blockquote draggable="true"></blockquote>
    <blockquote></blockquote>
    <blockquote></blockquote>
    <div id="log">Drag the orange square over the blue square then the fuchsia square, then release it.</div>
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
        "byte_end": 103,
        "byte_start": 80,
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
        "byte_end": 426,
        "byte_start": 363,
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
        "byte_end": 509,
        "byte_start": 440,
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
        "byte_end": 554,
        "byte_start": 523,
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
  "source_name": "html/editing/dnd/target-origin/002-manual.html"
}
```

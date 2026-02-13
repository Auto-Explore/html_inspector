# html/editing/dnd/target-origin/001-manual.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/001-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>allowTargetOrigin syntax</title>
    <style type="text/css">
blockquote { height: 100px; width: 100px; background: orange; margin: 0; padding: 0; }
    </style>
    <script type="text/javascript" src="/resources/testharness.js"></script>
    <script type="text/javascript" src="/resources/testharnessreport.js"></script>
    <script type="text/javascript">
setup(function () {},{explicit_done:true});
window.onload = function () {
  document.getElementsByTagName('blockquote')[0].ondragstart = function (e) {
    test(function() {
      assert_true( !!e.dataTransfer.allowTargetOrigin );
    }, 'allowTargetOrigin should be supported' );
    test(function() {
      assert_throws_js( TypeError, function () { e.dataTransfer.allowTargetOrigin(); } );
    }, 'no parameter should throw TypeError' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin(''); } );
    }, 'empty string should be an invalid URL' );
    test(function() {
      e.dataTransfer.allowTargetOrigin('*');
    }, '* should be a valid URL' );
    test(function() {
      e.dataTransfer.allowTargetOrigin('/');
    }, '/ should be a valid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('/foo'); } );
    }, '/foo should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('foo'); } );
    }, 'foo should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('//foo'); } );
    }, '//foo should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('http://'); } );
    }, 'http:// should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('http://*'); } );
    }, 'http://* should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('http://foo*'); } );
    }, 'http://foo* should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('http://foo.*'); } );
    }, 'http://foo.* should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('http://*.foo'); } );
    }, 'http://*.foo should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('http://foo:bar'); } );
    }, 'http://foo:bar should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('http://foo:bar@'); } );
    }, 'http://foo:bar@ should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('file:'); } );
    }, 'file: should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('file://'); } );
    }, 'file:// should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('data:'); } );
    }, 'data: should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('data:text/html'); } );
    }, 'data:text/html should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('file://localhost/'); } );
    }, 'file://localhost/ should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('file:///'); } );
    }, 'file:/// should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('data:text/html,'); } );
    }, 'data:text/html, should be an invalid URL' );
    test(function() {
      assert_throws_dom( 'SYNTAX_ERR', function () { e.dataTransfer.allowTargetOrigin('javascript:'); } );
    }, 'javascript: should be an invalid URL' );
    test(function() {
      e.dataTransfer.allowTargetOrigin('http://foo');
    }, 'http://foo should be a valid URL' );
    test(function() {
      e.dataTransfer.allowTargetOrigin('http://foo.bar');
    }, 'http://foo.bar should be a valid URL' );
    test(function() {
      e.dataTransfer.allowTargetOrigin('http://foo/bar');
    }, 'http://foo/bar should be a valid URL' );
    test(function() {
      e.dataTransfer.allowTargetOrigin('http://foo:123');
    }, 'http://foo:123 should be a valid URL' );
    test(function() {
      e.dataTransfer.allowTargetOrigin('http://foo:bar@baz');
    }, 'http://foo:bar@baz should be a valid URL' );
    test(function() {
      e.dataTransfer.allowTargetOrigin('http://foo:bar@baz:123/qux');
    }, 'http://foo:bar@baz:123/qux should be a valid URL' );
    done();
  };
};
    </script>
  </head>
  <body>
    <blockquote draggable="true"></blockquote>
    <div id="log">Drag the orange square above until the drag placeholder appears, then release it.</div>
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
        "byte_end": 271,
        "byte_start": 208,
        "col": 5,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 354,
        "byte_start": 285,
        "col": 5,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 399,
        "byte_start": 368,
        "col": 5,
        "line": 10
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
  "source_name": "html/editing/dnd/target-origin/001-manual.html"
}
```

# html/editing/editing-0/making-entire-documents-editable-the-designmode-idl-attribute/user-interaction-editing-designMode.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/making-entire-documents-editable-the-designmode-idl-attribute/user-interaction-editing-designMode.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
 <head>
  <title>Editing: designMode attribute test</title>
  <link rel="author" title="Baidu" href="mailto: guopengcheng@baidu.com"/>
  <link rel="help" href="https://html.spec.whatwg.org/multipage/#making-entire-documents-editable:-the-designmode-idl-attribute"/>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <div id="log"></div>
 </head>
 <body>
  <script type="text/javascript">
   test(function() {
    assert_equals(document.designMode, "off", "check for designMode value");
    assert_true(document.queryCommandSupported("delete"));
    assert_false(document.queryCommandEnabled("delete"));
   }, "initial designMode attribute");
   document.designMode="on";
   test(function() {
    assert_equals(document.designMode, "on", "check for designMode value");
    assert_true(document.queryCommandSupported("delete"));
    assert_true(document.queryCommandEnabled("delete"));
   }, "set designMode = \"on\"");
   document.designMode="off";
   test(function() {
    assert_equals(document.designMode,"off", "check for designMode value");
    assert_true(document.queryCommandSupported("delete"));
    assert_false(document.queryCommandEnabled("delete"));
   }, "set designMode = \"off\"");
  </script>
 </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.invalid",
      "message": "Bad value “mailto: guopengcheng@baidu.com” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 157,
        "byte_start": 85,
        "col": 3,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “head”.",
      "severity": "Error",
      "span": {
        "byte_end": 430,
        "byte_start": 423,
        "col": 2,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.body.already_open",
      "message": "Start tag “body” seen but an element of the same type was already open.",
      "severity": "Error",
      "span": {
        "byte_end": 438,
        "byte_start": 432,
        "col": 2,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 438,
        "byte_start": 432,
        "col": 2,
        "line": 11
      }
    }
  ],
  "source_name": "html/editing/editing-0/making-entire-documents-editable-the-designmode-idl-attribute/user-interaction-editing-designMode.html"
}
```

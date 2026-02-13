# html/editing/editing-0/spelling-and-grammar-checking/user-interaction-editing-spellcheck.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/spelling-and-grammar-checking/user-interaction-editing-spellcheck.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
 <head>
  <title>Editing: spellcheck attribute test</title>
  <link rel="author" title="Baidu" href="mailto: guopengcheng@baidu.com"/>
  <link rel="help" href="https://html.spec.whatwg.org/multipage/#spelling-and-grammar-checking"/>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
  <div id="log"></div>
  <textarea id="testText1" spellcheck="true">Test textarea with spellcheck is true</textarea>
  <textarea id="testText2" spellcheck="false">Test textarea with spellcheck is false</textarea>
  <script type="text/javascript">
   test(function() {
    assert_true(document.getElementById("testText1").spellcheck, "check for testText1 spellcheck value");
    assert_false(document.getElementById("testText2").spellcheck, "check for testText2 spellcheck value");
   }, "Getting spellcheck IDL attribute");
   test(function() {
    var testElement = document.createElement("testElement");
    testElement.contentEditable = true;
    testElement.spellcheck = true;
    assert_true(testElement.spellcheck, "check for testElement.spellcheck value");
    assert_equals(testElement.getAttribute("spellcheck"), "true");
   }, "Setting spellcheck IDL attribute to true");
   test(function() {
    var testElement = document.createElement("testElement");
    testElement.contentEditable = true;
    testElement.spellcheck = false;
    assert_false(testElement.spellcheck, "check for testText2 spellcheck value");
    assert_equals(testElement.getAttribute("spellcheck"), "false");
   }, "Setting spellcheck IDL attribute to false");
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
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 629,
        "byte_start": 598,
        "col": 3,
        "line": 14
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
  "source_name": "html/editing/editing-0/spelling-and-grammar-checking/user-interaction-editing-spellcheck.html"
}
```

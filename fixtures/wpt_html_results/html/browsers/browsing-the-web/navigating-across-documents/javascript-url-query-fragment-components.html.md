# html/browsers/browsing-the-web/navigating-across-documents/javascript-url-query-fragment-components.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-query-fragment-components.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title> javascript url with query and fragment components </title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
var a = null;
var b = null;
var c = null;
</script>

<iframe id="a" src='javascript:"nope" ? "yep" : "what";'></iframe>
<iframe id="b" src='javascript:"wrong"; // # %0a "ok";'></iframe>
<iframe id="c" src='javascript:"%252525 ? %252525 # %252525"'></iframe>

<script>
var t = async_test("iframes with javascript src");
function check(id, expected) {
  assert_equals(
    document.getElementById(id).contentDocument.body.textContent,
    expected);
}
onload = t.step_func(function() {
  check("a", "yep");
  check("b", "ok");
  check("c", "%2525 ? %2525 # %2525");
  t.done();
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “javascript:\"nope\" ? \"yep\" : \"what\";” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 308,
        "byte_start": 251,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “javascript:\"wrong\"; // # %0a \"ok\";” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 374,
        "byte_start": 318,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “javascript:\"%252525 ? %252525 # %252525\"” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 446,
        "byte_start": 384,
        "col": 1,
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-query-fragment-components.html"
}
```

# html/browsers/browsing-the-web/navigating-across-documents/javascript-url-return-value-handling.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-return-value-handling.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Test that javascript: evaluation only performs a navigation to the
  result when the result is a string value.</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<iframe src="javascript:'1'"></iframe>
<iframe src="javascript:1"></iframe>
<iframe src="javascript:({ toString: function() { return '1'; } })"></iframe>
<iframe src="javascript:undefined"></iframe>
<iframe src="javascript:null"></iframe>
<iframe src="javascript:true"></iframe>
<iframe src="javascript:new String('1')"></iframe>
<script>
  var t = async_test();
  onload = t.step_func_done(function() {
    assert_equals(frames[0].document.documentElement.textContent,
                  "1", "string return should cause navigation");
    // The rest of the test is disabled for now, until
    //  https://github.com/whatwg/html/issues/1895 gets sorted out
/*
    assert_equals(frames[1].document.documentElement.textContent,
                  "", "number return should not cause navigation");
    assert_equals(frames[2].document.documentElement.textContent,
                  "", "object return should not cause navigation");
    assert_equals(frames[3].document.documentElement.textContent,
                  "", "undefined return should not cause navigation");
    assert_equals(frames[4].document.documentElement.textContent,
                  "", "null return should not cause navigation");
    assert_equals(frames[5].document.documentElement.textContent,
                  "", "null return should not cause navigation");
    assert_equals(frames[6].document.documentElement.textContent,
                  "", "String object return should not cause navigation");
*/
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “javascript:({ toString: function() { return '1'; } })” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 409,
        "byte_start": 341,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “javascript:new String('1')” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 585,
        "byte_start": 544,
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-return-value-handling.html"
}
```

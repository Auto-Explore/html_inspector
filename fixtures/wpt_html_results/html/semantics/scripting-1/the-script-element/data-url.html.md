# html/semantics/scripting-1/the-script-element/data-url.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/data-url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Test data URL and scripts errors</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
  setup({allow_uncaught_exception:true});
  async_test(function(t) {
    var counter = 1
    window.onerror = t.step_func((message, url, lineno, colno, e) => {
      // Test that error is not muted as data URLs have a response type of "default"
      // and errors should only be muted if the response type is "opaque" or "opaqueredirect"
      assert_not_equals(message, "Script error.")
      assert_not_equals(url, null);
      assert_not_equals(url, "");
      assert_equals(typeof lineno, "number");
      assert_not_equals(lineno, 0);
      assert_equals(typeof colno, "number");
      assert_not_equals(colno, 0);
      assert_equals(typeof e, "number")
      assert_equals(e, counter)
      if (counter == 3) {
        t.done()
      }
      counter++
    });
  });
</script>
<script src="data:,throw 1"></script>
<script src="data:,throw 2" crossorigin></script>
<script src="data:,throw 3" crossorigin=use-credentials></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.src.invalid",
      "message": "Bad value “data:,throw 1” for attribute “src” on element “script”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1033,
        "byte_start": 1005,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.script.src.invalid",
      "message": "Bad value “data:,throw 2” for attribute “src” on element “script”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1083,
        "byte_start": 1043,
        "col": 1,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.script.src.invalid",
      "message": "Bad value “data:,throw 3” for attribute “src” on element “script”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1149,
        "byte_start": 1093,
        "col": 1,
        "line": 32
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
  "source_name": "html/semantics/scripting-1/the-script-element/data-url.html"
}
```

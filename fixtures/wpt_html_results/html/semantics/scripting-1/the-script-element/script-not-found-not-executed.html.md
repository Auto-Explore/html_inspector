# html/semantics/scripting-1/the-script-element/script-not-found-not-executed.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-not-found-not-executed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title></title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>var test1_token = "script not executed";</script>
<script src="script-not-found-not-executed.py"></script>
<script>
test(function(){
    assert_equals(test1_token, "script not executed");
}, "Script that 404");
</script>
<script>var test2_token = "script not executed";</script>
<script src="script-not-found-not-executed-2.py"></script>
<script>
test(function(){
    assert_equals(test2_token, "script executed");
}, "Script that does not 404");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 46,
        "byte_start": 39,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-not-found-not-executed.html"
}
```

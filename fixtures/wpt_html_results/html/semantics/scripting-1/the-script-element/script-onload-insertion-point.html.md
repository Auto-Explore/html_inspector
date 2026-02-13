# html/semantics/scripting-1/the-script-element/script-onload-insertion-point.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-onload-insertion-point.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Test that the insertion point is defined in the load event of a parser-inserted script.</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
  var t = async_test("");
  var writeDone = t.step_func_done(function(text) {
    assert_equals(text, "Some text");
  });
</script>
<iframe src="support/script-onload-insertion-point-helper.html"></iframe>
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-onload-insertion-point.html"
}
```

# html/semantics/scripting-1/the-script-element/script-charset-03.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-charset-03.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<meta charset="utf-8">
<title>Script changing @charset</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#scriptingLanguages">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
async_test(function() {
  var s = document.createElement("script");
  s.src = "external-script-windows1250.js";
  s.charset = "windows-1250";
  document.body.appendChild(s);
  s.charset = "utf-8";
  window.onload = this.step_func_done(function() {
    assert_equals(window.getSomeString(), "\u015b\u0107\u0105\u017c\u017a");
  });
})
</script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-charset-03.html"
}
```

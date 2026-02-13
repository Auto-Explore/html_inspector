# html/browsers/browsing-the-web/navigating-across-documents/source/navigate-child-function-src.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/source/navigate-child-function-src.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Set src from a function called from a parent</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<iframe src="support/set-parent-src.html"></iframe>
<script>
async_test(function() {
  onload = this.step_func(function() {
    var fr = document.querySelector("iframe")
    fr.contentWindow.go()
    fr.onload = this.step_func_done(function() {
      assert_equals(fr.contentDocument.referrer, document.URL)
    })
  })
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/source/navigate-child-function-src.html"
}
```

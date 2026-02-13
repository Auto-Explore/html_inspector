# html/browsers/windows/noreferrer-null-opener.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/noreferrer-null-opener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>rel=noreferrer nullifies window.opener</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  async_test(function(t) {
    localStorage.clear()

    var hyperlink = document.body.appendChild(document.createElement("a"))
    hyperlink.rel = "noreferrer"
    hyperlink.target = "_blank"
    hyperlink.href = "resources/window-opener.html"
    hyperlink.click()
    document.body.removeChild(hyperlink)

    addEventListener("storage", function(e) {
      t.step(function() {
        assert_equals(e.newValue, "null")
        localStorage.clear()
        t.done()
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
  "source_name": "html/browsers/windows/noreferrer-null-opener.html"
}
```

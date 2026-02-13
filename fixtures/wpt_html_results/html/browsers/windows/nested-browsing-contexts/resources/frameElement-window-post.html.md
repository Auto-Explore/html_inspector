# html/browsers/windows/nested-browsing-contexts/resources/frameElement-window-post.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/nested-browsing-contexts/resources/frameElement-window-post.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8"/>
<title>Testcase 3: frameElement attribute must return null if the container\'s document does not have the same effective script origin</title>
<script>
window.addEventListener("message", function (event) {
  try {
    var result = "window.frameElement = " + window.frameElement;
  } catch (e) {
    result = e.message;
  }
  event.source.postMessage(JSON.stringify({name: "testcase3", result: result}),
                           "*");
}, false);
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
  "source_name": "html/browsers/windows/nested-browsing-contexts/resources/frameElement-window-post.html"
}
```

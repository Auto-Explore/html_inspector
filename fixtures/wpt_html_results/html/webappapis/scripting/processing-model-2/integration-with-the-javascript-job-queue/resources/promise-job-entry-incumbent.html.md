# html/webappapis/scripting/processing-model-2/integration-with-the-javascript-job-queue/resources/promise-job-entry-incumbent.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/integration-with-the-javascript-job-queue/resources/promise-job-entry-incumbent.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Incumbent page used as a test helper</title>

<iframe src="relevant/relevant.html" id="r"></iframe>
<iframe src="current/current.html" id="c"></iframe>

<script>
  const relevant = document.querySelector("#r");
  const current = document.querySelector("#c");

  window.runWindowOpenVeryIndirectly = (...args) => {
    return current.contentWindow.open.call(relevant.contentWindow, ...args);
  };
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
  "source_name": "html/webappapis/scripting/processing-model-2/integration-with-the-javascript-job-queue/resources/promise-job-entry-incumbent.html"
}
```

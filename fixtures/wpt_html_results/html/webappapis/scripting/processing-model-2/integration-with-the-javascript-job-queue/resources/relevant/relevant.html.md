# html/webappapis/scripting/processing-model-2/integration-with-the-javascript-job-queue/resources/relevant/relevant.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/integration-with-the-javascript-job-queue/resources/relevant/relevant.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Relevant page used as a test helper</title>

<script>
// promise-job-incumbent will end up posting a message to here. We need to signal back the "source".

window.onmessage = e => {
  const testId = e.data;
  const sourceURL = e.source.document.URL;

  window.dispatchEvent(new CustomEvent("messagereceived", { detail: [testId, sourceURL] }));
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
  "source_name": "html/webappapis/scripting/processing-model-2/integration-with-the-javascript-job-queue/resources/relevant/relevant.html"
}
```

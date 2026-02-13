# html/webappapis/scripting/processing-model-2/integration-with-the-javascript-job-queue/resources/promise-job-incumbent-incumbent.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/integration-with-the-javascript-job-queue/resources/promise-job-incumbent-incumbent.html",
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

  window.runWindowPostMessageVeryIndirectly = (...args) => {
    return current.contentWindow.postMessage.call(relevant.contentWindow, ...args);
  };

  // This tests the backup incumbent settings object stack scenario, by avoiding putting user code on the stack.
  window.runWindowPostMessageVeryIndirectlyWithNoUserCode = (promise, promiseMethod, ...args) => {
    const runWindowPostMessage = current.contentWindow.postMessage.bind(relevant.contentWindow, ...args);
    promise[promiseMethod](runWindowPostMessage);
  };

  window.resolveThenableThatRunsWindowPostMessageVeryIndirectlyWithNoUserCode = (...args) => {
    Promise.resolve({
      then: current.contentWindow.postMessage.bind(relevant.contentWindow, ...args)
    });
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
  "source_name": "html/webappapis/scripting/processing-model-2/integration-with-the-javascript-job-queue/resources/promise-job-incumbent-incumbent.html"
}
```

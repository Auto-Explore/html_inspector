# html/browsers/browsing-the-web/unloading-documents/beforeunload-canceling-1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/beforeunload-canceling-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Support page for beforeunload-canceling.html</title>

<h1>If this goes away, it navigated</h1>

<script>
"use strict";

window.runTest = (t, { valueToReturn, expectCancelation, setReturnValue, expectedReturnValue, cancel }) => {
  window.onbeforeunload = t.step_func(e => {
    if (cancel) {
      e.preventDefault();
    }

    if (setReturnValue !== undefined) {
      e.returnValue = setReturnValue;
    }

    return valueToReturn;
  });

  const listener = t.step_func(e => {
    top.assert_equals(e.defaultPrevented, expectCancelation, "canceled");
    top.assert_equals(e.returnValue, expectedReturnValue, "returnValue");
    window.onbeforeunload = null;

    t.done();
  });

  window.addEventListener("beforeunload", listener);

  window.location.href = "about:blank";
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/beforeunload-canceling-1.html"
}
```

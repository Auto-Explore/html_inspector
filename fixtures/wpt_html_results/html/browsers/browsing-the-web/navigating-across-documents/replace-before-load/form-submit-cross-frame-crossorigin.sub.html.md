# html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/form-submit-cross-frame-crossorigin.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/form-submit-cross-frame-crossorigin.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>

<form target="the-frame">
  <input type="hidden" name="pushed">
</form>

<script>
"use strict";
promise_test(async t => {
  const startingHistoryLength = history.length;

  const form = document.querySelector("form");
  const frameEndingURL = changeURLHost(
    absoluteURL("resources/slow-message-source-with-history-and-location.html?pushed="),
    "{{hosts[][www]}}"
  );
  form.action = frameEndingURL;

  const frameStartingCode = `
    parent.postMessage({ historyLength: history.length, locationHref: location.href }, "*");
  `;

  const frameStartingURL = codeInjectorURL(frameStartingCode);
  const frame = insertIframe(t, frameStartingURL, "the-frame");
  t.add_cleanup(() => frame.remove()); // helps avoid waiting for the slow load to finish the tests
  assert_equals(history.length, startingHistoryLength, "Inserting frame must not change history.length");

  const frameBeforeLoadedMessage = await waitForMessage();
  assert_equals(frameBeforeLoadedMessage.historyLength, startingHistoryLength, "frame's starting history.length");
  assert_equals(frameBeforeLoadedMessage.locationHref, frame.src, "frame's starting location.href");

  form.submit();

  const frameAfterFormSubmitMessage = await waitForMessage();
  assert_equals(frameAfterFormSubmitMessage.historyLength, startingHistoryLength + 1, "frame's after-submit history.length");
  assert_equals(frameAfterFormSubmitMessage.locationHref, frameEndingURL, "frame's after-submit location.href");
}, "No replace before load, triggered by cross-iframe formElement.submit() [iframe is cross-origin]");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/form-submit-cross-frame-crossorigin.sub.html"
}
```

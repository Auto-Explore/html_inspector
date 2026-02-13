# html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/form-submit-popup-crossorigin.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/form-submit-popup-crossorigin.sub.html",
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

<form target="the-window">
  <input type="hidden" name="pushed">
</form>

<script>
"use strict";
promise_test(async t => {
  const form = document.querySelector("form");
  const wEndingURL = changeURLHost(
    absoluteURL("resources/slow-message-source-with-history-and-location.html?pushed="),
    "{{hosts[][www]}}"
  );
  form.action = wEndingURL;

  const wStartingCode = `
    opener.postMessage({ historyLength: history.length, locationHref: location.href }, "*");
  `;

  const wStartingURL = codeInjectorURL(wStartingCode);
  const w = window.open(wStartingURL, "the-window");
  t.add_cleanup(() => w.close());

  const wBeforeLoadedMessage = await waitForMessage();
  assert_equals(wBeforeLoadedMessage.historyLength, 1, "window's starting history.length");
  assert_equals(wBeforeLoadedMessage.locationHref, wStartingURL, "window's starting location.href");

  form.submit();

  const wAfterFormSubmitMessage = await waitForMessage();
  assert_equals(wAfterFormSubmitMessage.historyLength, 2, "window's after-submit history.length");
  assert_equals(wAfterFormSubmitMessage.locationHref, wEndingURL, "window's after-submit location.href");
}, "No replace before load, triggered by formElement.submit() in the opener window, after the opener has loaded [window is cross-origin]");
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/form-submit-popup-crossorigin.sub.html"
}
```

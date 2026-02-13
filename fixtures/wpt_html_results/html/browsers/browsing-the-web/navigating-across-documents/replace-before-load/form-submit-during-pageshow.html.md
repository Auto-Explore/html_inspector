# html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/form-submit-during-pageshow.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/form-submit-during-pageshow.html",
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

<body>
<script>
"use strict";
promise_test(async t => {
  const sentinelIframe = await setupSentinelIframe(t);
  const startingHistoryLength = history.length;

  const code = `
    window.onpageshow = () => {
      const form = document.createElement("form");
      form.action = "/common/blank.html";

      const input = document.createElement("input");
      input.type = "hidden";
      input.name = "thereplacement";
      form.append(input);

      document.body.append(form);
      form.submit();
    };
  `;

  const startURL = "resources/code-injector.html?pipe=sub(none)&code=" + encodeURIComponent(code);
  const afterReplacementURL = "/common/blank.html?thereplacement=";
  const iframe = insertIframe(t, startURL);

  assert_equals(history.length, startingHistoryLength, "Inserting the under-test iframe must not change history.length");

  await waitForLoadAllowingIntermediateLoads(t, iframe, afterReplacementURL);
  assert_equals(history.length, startingHistoryLength, "history.length must not change after waiting for the replacement");

  await checkSentinelIframe(t, sentinelIframe);
  assert_equals(history.length, startingHistoryLength, "history.length must not change after checking the sentinel iframe");
}, "Replace during the pageshow event, triggered by formElement.submit()");
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/form-submit-during-pageshow.html"
}
```
